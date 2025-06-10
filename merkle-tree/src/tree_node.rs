use std::str::FromStr;

use serde::{Deserialize, Serialize};
use solana_program::{hash::hashv, pubkey::Pubkey};
use solana_sdk::hash::Hash;

use crate::csv_entry::CsvEntry;
/// Represents the claim information for an account.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TreeNode {
    /// Pubkey of the claimant; will be responsible for signing the claim
    pub claimant: Pubkey,
    /// Claimant's proof of inclusion in the Merkle Tree
    pub proof: Option<Vec<[u8; 32]>>,
    /// Total amount unlocked
    pub total_unlocked: u64,
    /// Total amount locked
    pub total_locked: u64,
}

impl TreeNode {
    pub fn hash(&self) -> Hash {
        hashv(&[
            &self.claimant.to_bytes(),
            &self.amount_unlocked().to_le_bytes(),
            &self.amount_locked().to_le_bytes(),
        ])
    }

    /// Return total amount of locked and unlocked amount for this claimant
    pub fn total_amount(&self) -> u64 {
        self.amount_unlocked()
            .checked_add(self.amount_locked())
            .unwrap()
    }

    /// Get total amount of unlocked tokens for this claimant
    pub fn amount_unlocked(&self) -> u64 {
        self.total_unlocked
    }

    /// Get total amount of locked tokens for this claimant
    pub fn amount_locked(&self) -> u64 {
        self.total_locked
    }
}

impl From<CsvEntry> for TreeNode {
    fn from(entry: CsvEntry) -> Self {
        let mut node = Self {
            claimant: Pubkey::from_str(entry.pubkey.as_str()).unwrap(),
            proof: None,
            total_unlocked: 0,
            total_locked: 0,
        };

        // CSV entry uses UI amounts; we convert to native amounts here
        node.total_unlocked = entry.amount_unlocked;
        node.total_locked = entry.amount_locked;

        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_tree_node() {
        let tree_node = TreeNode {
            claimant: Pubkey::default(),
            proof: None,
            total_unlocked: 0,
            total_locked: 0,
        };
        let serialized = serde_json::to_string(&tree_node).unwrap();
        let deserialized: TreeNode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tree_node, deserialized);
    }
}
