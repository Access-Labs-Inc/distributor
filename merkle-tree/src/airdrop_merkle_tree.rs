use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
    result,
};

use access_merkle_verify::verify;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use solana_program::{hash::hashv, pubkey::Pubkey};

use crate::{
    csv_entry::CsvEntry,
    error::{MerkleTreeError, MerkleTreeError::MerkleValidationError},
    merkle_tree::MerkleTree,
    tree_node::TreeNode,
    utils::{get_max_total_claim, get_proof},
};

// We need to discern between leaf and intermediate nodes to prevent trivial second
// pre-image attacks.
// https://flawed.net.nz/2018/02/21/attacking-merkle-trees-with-a-second-preimage-attack
const LEAF_PREFIX: &[u8] = &[0];

/// Merkle Tree which will be used to distribute tokens to claimants.
/// Contains all the information necessary to verify claims against the Merkle Tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirdropMerkleTree {
    /// The merkle root, which is uploaded on-chain
    pub merkle_root: [u8; 32],
    pub max_num_nodes: u64,
    pub max_total_claim: u64,
    pub tree_nodes: Vec<TreeNode>,
}

pub type Result<T> = result::Result<T, MerkleTreeError>;

impl AirdropMerkleTree {
    pub fn new(tree_nodes: Vec<TreeNode>) -> Result<Self> {
        // Combine tree nodes with the same claimant, while retaining original order
        let mut tree_nodes_map: IndexMap<Pubkey, TreeNode> = IndexMap::new();
        for tree_node in tree_nodes {
            let claimant = tree_node.claimant;
            tree_nodes_map
                .entry(claimant)
                .and_modify(|n| {
                    println!("duplicate claimant {} found, combining", n.claimant);
                    n.total_unlocked = n
                        .total_unlocked
                        .checked_add(tree_node.total_unlocked)
                        .unwrap();
                    n.total_locked = n.total_locked.checked_add(tree_node.total_locked).unwrap();
                })
                .or_insert_with(|| tree_node); // If not exists, insert a new entry
        }

        // Convert IndexMap back to Vec while preserving the order
        let mut tree_nodes: Vec<TreeNode> = tree_nodes_map.values().cloned().collect();

        // Sort by claimant pubkey for deterministic order
        tree_nodes.sort_by_key(|n| n.claimant);

        let hashed_nodes = tree_nodes
            .iter()
            .map(|claim_info| claim_info.hash().to_bytes())
            .collect::<Vec<_>>();

        let tree = MerkleTree::new(&hashed_nodes[..], true);

        for (i, tree_node) in tree_nodes.iter_mut().enumerate() {
            tree_node.proof = Some(get_proof(&tree, i));
        }

        let max_total_claim = get_max_total_claim(tree_nodes.as_ref());
        let tree = AirdropMerkleTree {
            merkle_root: tree
                .get_root()
                .ok_or(MerkleTreeError::MerkleRootError)?
                .to_bytes(),
            max_num_nodes: tree_nodes.len() as u64,
            max_total_claim,
            tree_nodes,
        };

        println!(
            "created merkle tree with {} nodes and max total claim of {}",
            tree.max_num_nodes, tree.max_total_claim
        );
        tree.validate()?;
        Ok(tree)
    }

    /// Load a merkle tree from a csv path
    pub fn new_from_csv(path: &PathBuf) -> Result<Self> {
        let csv_entries = CsvEntry::new_from_file(path)?;
        let tree_nodes: Vec<TreeNode> = csv_entries.into_iter().map(TreeNode::from).collect();
        let tree = Self::new(tree_nodes)?;
        Ok(tree)
    }

    /// Load a serialized merkle tree from file path
    pub fn new_from_file(path: &PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let tree: AirdropMerkleTree = serde_json::from_reader(reader)?;

        Ok(tree)
    }

    /// Write a merkle tree to a filepath
    pub fn write_to_file(&self, path: &PathBuf) {
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    pub fn get_node(&self, claimant: &Pubkey) -> TreeNode {
        for i in self.tree_nodes.iter() {
            if i.claimant == *claimant {
                return i.clone();
            }
        }

        panic!("Claimant not found in tree");
    }

    fn validate(&self) -> Result<()> {
        // The Merkle tree can be at most height 32, implying a max node count of 2^32 - 1
        if self.max_num_nodes > 2u64.pow(32) - 1 {
            return Err(MerkleValidationError(format!(
                "Max num nodes {} is greater than 2^32 - 1",
                self.max_num_nodes
            )));
        }

        // validate that the length is equal to the max_num_nodes
        if self.tree_nodes.len() != self.max_num_nodes as usize {
            return Err(MerkleValidationError(format!(
                "Tree nodes length {} does not match max_num_nodes {}",
                self.tree_nodes.len(),
                self.max_num_nodes
            )));
        }

        // validate that there are no duplicate claimants
        let unique_nodes: HashSet<_> = self.tree_nodes.iter().map(|n| n.claimant).collect();

        if unique_nodes.len() != self.tree_nodes.len() {
            return Err(MerkleValidationError(
                "Duplicate claimants found".to_string(),
            ));
        }

        // validate that sum is equal to max_total_claim
        let sum = get_max_total_claim(&self.tree_nodes);

        if sum != self.max_total_claim {
            return Err(MerkleValidationError(format!(
                "Tree nodes sum {} does not match max_total_claim {}",
                sum, self.max_total_claim
            )));
        }

        if self.verify_proof().is_err() {
            return Err(MerkleValidationError(
                "Merkle root is invalid given nodes".to_string(),
            ));
        }

        Ok(())
    }

    /// verify that the leaves of the merkle tree match the nodes
    pub fn verify_proof(&self) -> Result<()> {
        let root = self.merkle_root;

        // Recreate root given nodes
        let hashed_nodes: Vec<[u8; 32]> = self
            .tree_nodes
            .iter()
            .map(|n| n.hash().to_bytes())
            .collect();
        let mk = MerkleTree::new(&hashed_nodes[..], true);

        assert_eq!(
            mk.get_root()
                .ok_or(MerkleValidationError("invalid merkle proof".to_string()))?
                .to_bytes(),
            root
        );

        // Verify each node against the root
        for (i, _node) in hashed_nodes.iter().enumerate() {
            let node = hashv(&[LEAF_PREFIX, &hashed_nodes[i]]);
            let proof = get_proof(&mk, i);

            if !verify(proof, root, node.to_bytes()) {
                return Err(MerkleValidationError("invalid merkle proof".to_string()));
            }
        }

        Ok(())
    }

    // Converts Merkle Tree to a map for faster key access
    pub fn convert_to_hashmap(&self) -> HashMap<Pubkey, TreeNode> {
        self.tree_nodes
            .iter()
            .map(|n| (n.claimant, n.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use solana_program::{pubkey, pubkey::Pubkey};
    use solana_sdk::{
        signature::{EncodableKey, Keypair},
        signer::Signer,
    };

    use super::*;

    pub fn new_test_key() -> Pubkey {
        let kp = Keypair::new();
        let out_path = format!("./test_keys/{}.json", kp.pubkey());

        kp.write_to_file(out_path)
            .expect("Failed to write to signer");

        kp.pubkey()
    }

    fn new_test_merkle_tree(num_nodes: u64, path: &PathBuf) {
        let mut tree_nodes = vec![];

        fn rand_balance() -> u64 {
            rand::random::<u64>() % 100 * u64::pow(10, 9)
        }

        for _ in 0..num_nodes {
            // choose amount unlocked and amount locked as a random u64 between 0 and 100
            tree_nodes.push(TreeNode {
                claimant: new_test_key(),
                proof: None,
                total_unlocked: rand_balance(),
                total_locked: rand_balance(),
            });
        }

        let merkle_tree = AirdropMerkleTree::new(tree_nodes).unwrap();

        merkle_tree.write_to_file(path);
    }

    #[test]
    fn test_verify_new_merkle_tree() {
        let tree_nodes = vec![TreeNode {
            claimant: Pubkey::default(),
            proof: None,
            total_unlocked: 2,
            total_locked: 3,
        }];
        let merkle_tree = AirdropMerkleTree::new(tree_nodes).unwrap();
        assert!(merkle_tree.verify_proof().is_ok(), "verify failed");
    }

    #[test]
    fn test_write_merkle_distributor_to_file() {
        // create a merkle root from 3 tree nodes and write it to file, then read it
        let tree_nodes = vec![
            TreeNode {
                claimant: pubkey!("FLYqJsmJ5AGMxMxK3Qy1rSen4ES2dqqo6h51W3C1tYS"),
                proof: None,
                total_unlocked: (100 * u64::pow(10, 9)),
                total_locked: (100 * u64::pow(10, 9)),
            },
            TreeNode {
                claimant: pubkey!("EDGARWktv3nDxRYjufjdbZmryqGXceaFPoPpbUzdpqED"),
                proof: None,
                total_unlocked: 100 * u64::pow(10, 9),
                total_locked: (100 * u64::pow(10, 9)),
            },
            TreeNode {
                claimant: pubkey!("EDGARWktv3nDxRYjufjdbZmryqGXceaFPoPpbUzdpqEH"),
                proof: None,
                total_locked: (100 * u64::pow(10, 9)),
                total_unlocked: (100 * u64::pow(10, 9)),
            },
        ];

        let merkle_distributor_info = AirdropMerkleTree::new(tree_nodes).unwrap();
        let path = PathBuf::from("merkle_tree.json");

        // serialize merkle distributor to file
        merkle_distributor_info.write_to_file(&path);
        // now test we can successfully read from file
        let merkle_distributor_read: AirdropMerkleTree =
            AirdropMerkleTree::new_from_file(&path).unwrap();

        assert_eq!(merkle_distributor_read.tree_nodes.len(), 3);
    }

    #[test]
    fn test_new_test_merkle_tree() {
        new_test_merkle_tree(100, &PathBuf::from("merkle_tree_test_csv.json"));
    }

    // Test creating a merkle tree from Tree Nodes, where claimants are not unique
    #[test]
    fn test_new_merkle_tree_duplicate_claimants() {
        let duplicate_pubkey = Pubkey::new_unique();
        let tree_nodes = vec![
            TreeNode {
                claimant: duplicate_pubkey,
                proof: None,
                total_unlocked: 10,
                total_locked: 20,
            },
            TreeNode {
                claimant: duplicate_pubkey,
                proof: None,
                total_unlocked: 1,
                total_locked: 2,
            },
            TreeNode {
                claimant: Pubkey::new_unique(),
                proof: None,
                total_unlocked: 0,
                total_locked: 0,
            },
        ];

        let tree = AirdropMerkleTree::new(tree_nodes).unwrap();
        // Assert that the merkle distributor correctly combines the two tree nodes
        assert_eq!(tree.tree_nodes.len(), 2);
        assert_eq!(tree.tree_nodes[0].total_unlocked, 11);
        assert_eq!(tree.tree_nodes[0].total_locked, 22);
    }
}
