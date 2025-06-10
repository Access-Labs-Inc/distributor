extern crate access_merkle_tree;
extern crate merkle_distributor;

use std::path::PathBuf;

use access_merkle_tree::{
    airdrop_merkle_tree::AirdropMerkleTree,
    utils::{get_claim_status_pda, get_merkle_distributor_pda},
};
use anchor_lang::{prelude::Pubkey, AccountDeserialize, InstructionData, Key, ToAccountMetas};
use anchor_spl::token;
use clap::{Parser, Subcommand};
use merkle_distributor::state::merkle_distributor::MerkleDistributor;
use solana_program::instruction::Instruction;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction, signature::read_keypair_file, signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    /// Airdrop version
    #[clap(long, env, default_value_t = 0)]
    pub airdrop_version: u64,

    /// SPL Mint address
    #[clap(long, env)]
    pub mint: Pubkey,

    /// RPC url
    #[clap(long, env)]
    pub rpc_url: String,

    /// Program id
    #[clap(long, env, default_value_t = merkle_distributor::id())]
    pub program_id: Pubkey,

    /// Payer keypair
    #[clap(long, env)]
    pub keypair_path: PathBuf,

    /// Priority fee
    #[clap(long, env)]
    pub priority: Option<u64>,

    /// Clawback receiver token account
    #[clap(long, env)]
    pub clawback_receiver_token_account: Option<Pubkey>,
}

// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Claim unlocked tokens
    Claim(ClaimArgs),
    /// Create a new instance of a merkle distributor
    NewDistributor(NewDistributorArgs),
    /// Clawback tokens from merkle distributor
    Clawback(ClawbackArgs),
    /// Create a Merkle tree, given a CSV of recipients
    CreateMerkleTree(CreateMerkleTreeArgs),
    SetAdmin(SetAdminArgs),
    /// Print the derived distributor PDA
    DistributorPda,
}

#[derive(Parser, Debug)]
pub struct ClawbackArgs {
    /// Distributor PDA
    #[clap(long, env)]
    pub distributor: Pubkey,
}

// NewClaim and Claim subcommand args
#[derive(Parser, Debug)]
pub struct ClaimArgs {
    /// Merkle distributor path
    #[clap(long, env)]
    pub merkle_tree_path: PathBuf,
    /// Distributor PDA
    #[clap(long, env)]
    pub distributor: Pubkey,
}

// NewDistributor subcommand args
#[derive(Parser, Debug)]
pub struct NewDistributorArgs {
    /// Merkle distributor path
    #[clap(long, env)]
    pub merkle_tree_path: PathBuf,

    /// Lockup timestamp start
    #[clap(long, env)]
    pub start_vesting_ts: i64,

    /// Lockup timestamp end (unix timestamp)
    #[clap(long, env)]
    pub end_vesting_ts: i64,
}

#[derive(Parser, Debug)]
pub struct CreateMerkleTreeArgs {
    /// CSV path
    #[clap(long, env)]
    pub csv_path: PathBuf,

    /// Merkle tree out path
    #[clap(long, env)]
    pub merkle_tree_path: PathBuf,
}

#[derive(Parser, Debug)]
pub struct SetAdminArgs {
    /// Distributor PDA
    #[clap(long, env)]
    pub distributor: Pubkey,

    /// New admin
    #[clap(long, env)]
    pub new_admin: Pubkey,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::NewDistributor(new_distributor_args) => {
            process_new_distributor(&args, new_distributor_args);
        }
        Commands::Claim(claim_args) => {
            process_claim(&args, claim_args);
        }
        Commands::Clawback(clawback_args) => process_clawback(&args, clawback_args),
        Commands::CreateMerkleTree(merkle_tree_args) => {
            process_create_merkle_tree(merkle_tree_args);
        }
        Commands::SetAdmin(set_admin_args) => {
            process_set_admin(&args, set_admin_args);
        }
        Commands::DistributorPda => {
            process_distributor_pda(&args);
        }
    }
}

fn process_new_claim(args: &Args, claim_args: &ClaimArgs) {
    let keypair = read_keypair_file(&args.keypair_path).expect("Failed reading keypair file");
    let claimant = keypair.pubkey();
    println!("Claiming tokens for user {}...", claimant);

    let merkle_tree = AirdropMerkleTree::new_from_file(&claim_args.merkle_tree_path)
        .expect("failed to load merkle tree from file");

    // Get user's node in claim
    let node = merkle_tree.get_node(&claimant);

    let (claim_status_pda, _bump) =
        get_claim_status_pda(&args.program_id, &claimant, &claim_args.distributor);

    let client = RpcClient::new_with_commitment(&args.rpc_url, CommitmentConfig::confirmed());

    let claimant_ata = get_associated_token_address(&claimant, &args.mint);

    let mut ixs = vec![];

    match client.get_account(&claimant_ata) {
        Ok(_) => {}
        Err(e) => {
            // TODO: directly pattern match on error kind
            if e.to_string().contains("AccountNotFound") {
                println!("PDA does not exist. creating.");
                let ix =
                    create_associated_token_account(&claimant, &claimant, &args.mint, &token::ID);
                ixs.push(ix);
            } else {
                panic!("Error fetching PDA: {e}")
            }
        }
    }

    let new_claim_ix = Instruction {
        program_id: args.program_id,
        accounts: merkle_distributor::accounts::NewClaim {
            distributor: claim_args.distributor,
            claim_status: claim_status_pda,
            from: get_associated_token_address(&claim_args.distributor, &args.mint),
            to: claimant_ata,
            claimant,
            token_program: token::ID,
            system_program: solana_program::system_program::ID,
        }
        .to_account_metas(None),
        data: merkle_distributor::instruction::NewClaim {
            amount_unlocked: node.amount_unlocked(),
            amount_locked: node.amount_locked(),
            proof: node.proof.expect("proof not found"),
        }
        .data(),
    };

    ixs.push(new_claim_ix);

    let blockhash = client.get_latest_blockhash().unwrap();
    let tx =
        Transaction::new_signed_with_payer(&ixs, Some(&claimant.key()), &[&keypair], blockhash);

    let signature = client
        .send_and_confirm_transaction_with_spinner(&tx)
        .unwrap();
    println!("successfully created new claim with signature {signature:#?}");
}

fn process_claim(args: &Args, claim_args: &ClaimArgs) {
    let keypair = read_keypair_file(&args.keypair_path).expect("Failed reading keypair file");
    let claimant = keypair.pubkey();

    let priority_fee = args.priority.unwrap_or(0);

    println!("args: {:?}", args);

    let (claim_status_pda, _bump) =
        get_claim_status_pda(&args.program_id, &claimant, &claim_args.distributor);
    println!("claim pda: {claim_status_pda}");

    let client = RpcClient::new_with_commitment(&args.rpc_url, CommitmentConfig::confirmed());

    match client.get_account(&claim_status_pda) {
        Ok(_) => {}
        Err(e) => {
            // TODO: match on the error kind
            if e.to_string().contains("AccountNotFound") {
                println!("PDA does not exist. creating.");
                process_new_claim(args, claim_args);
            } else {
                panic!("error getting PDA: {e}")
            }
        }
    }

    let claimant_ata = get_associated_token_address(&claimant, &args.mint);

    let mut ixs = vec![];

    let claim_ix = Instruction {
        program_id: args.program_id,
        accounts: merkle_distributor::accounts::ClaimLocked {
            distributor: claim_args.distributor,
            claim_status: claim_status_pda,
            from: get_associated_token_address(&claim_args.distributor, &args.mint),
            to: claimant_ata,
            claimant,
            token_program: token::ID,
        }
        .to_account_metas(None),
        data: merkle_distributor::instruction::ClaimLocked {}.data(),
    };
    ixs.push(claim_ix);

    if priority_fee > 0 {
        let instruction = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);
        ixs.push(instruction);
        println!(
            "Added priority fee instruction of {} microlamports",
            priority_fee
        );
    } else {
        println!("No priority fee added. Add one with --priority <microlamports u64>");
    }

    let blockhash = client.get_latest_blockhash().unwrap();
    let tx =
        Transaction::new_signed_with_payer(&ixs, Some(&claimant.key()), &[&keypair], blockhash);

    let signature = client
        .send_and_confirm_transaction_with_spinner(&tx)
        .unwrap();
    println!("successfully claimed tokens with signature {signature:#?}",);
}

fn check_distributor_onchain_matches(
    account: &Account,
    merkle_tree: &AirdropMerkleTree,
    new_distributor_args: &NewDistributorArgs,
    pubkey: Pubkey,
) -> Result<(), &'static str> {
    if let Ok(distributor) = MerkleDistributor::try_deserialize(&mut account.data.as_slice()) {
        if distributor.root != merkle_tree.merkle_root {
            return Err("root mismatch");
        }
        if distributor.max_total_claim != merkle_tree.max_total_claim {
            return Err("max_total_claim mismatch");
        }
        if distributor.max_num_nodes != merkle_tree.max_num_nodes {
            return Err("max_num_nodes mismatch");
        }

        if distributor.start_ts != new_distributor_args.start_vesting_ts {
            return Err("start_ts mismatch");
        }
        if distributor.end_ts != new_distributor_args.end_vesting_ts {
            return Err("end_ts mismatch");
        }
        if distributor.admin != pubkey {
            return Err("admin mismatch");
        }
    }
    Ok(())
}

fn process_new_distributor(args: &Args, new_distributor_args: &NewDistributorArgs) {
    let client = RpcClient::new_with_commitment(&args.rpc_url, CommitmentConfig::finalized());

    let keypair = read_keypair_file(&args.keypair_path).expect("Failed reading keypair file");
    let merkle_tree = AirdropMerkleTree::new_from_file(&new_distributor_args.merkle_tree_path)
        .expect("failed to read");
    let (distributor_pubkey, _bump) = get_merkle_distributor_pda(
        &args.program_id,
        &args.mint,
        &keypair.pubkey(),
        args.airdrop_version,
    );
    let token_vault = get_associated_token_address(&distributor_pubkey, &args.mint);

    if let Some(account) = client
        .get_account_with_commitment(&distributor_pubkey, CommitmentConfig::confirmed())
        .unwrap()
        .value
    {
        println!("merkle distributor account exists, checking parameters...");
        check_distributor_onchain_matches(
            &account,
            &merkle_tree,
            new_distributor_args,
            keypair.pubkey(),
        ).expect("merkle root on-chain does not match provided arguments! Confirm admin and clawback parameters to avoid loss of funds!");
    }

    println!(
        "creating new distributor with args: {new_distributor_args:#?}, address: {}",
        distributor_pubkey
    );

    let new_distributor_ix = Instruction {
        program_id: args.program_id,
        accounts: merkle_distributor::accounts::NewDistributor {
            distributor: distributor_pubkey,
            mint: args.mint,
            token_vault,
            creator: keypair.pubkey(),
            system_program: solana_program::system_program::id(),
            associated_token_program: spl_associated_token_account::ID,
            token_program: token::ID,
        }
        .to_account_metas(None),
        data: merkle_distributor::instruction::NewDistributor {
            version: args.airdrop_version,
            root: merkle_tree.merkle_root,
            max_total_claim: merkle_tree.max_total_claim,
            max_num_nodes: merkle_tree.max_num_nodes,
            start_vesting_ts: new_distributor_args.start_vesting_ts,
            end_vesting_ts: new_distributor_args.end_vesting_ts,
        }
        .data(),
    };

    let blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[new_distributor_ix],
        Some(&keypair.pubkey()),
        &[&keypair],
        blockhash,
    );

    // See comments on new_distributor instruction inside the program to ensure this transaction
    // didn't get frontrun.
    // If this fails, make sure to run it again.
    match client.send_and_confirm_transaction_with_spinner(&tx) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to create MerkleDistributor: {:?}", e);

            // double check someone didn't frontrun this transaction with a malicious merkle root
            if let Some(account) = client
                .get_account_with_commitment(&distributor_pubkey, CommitmentConfig::processed())
                .unwrap()
                .value
            {
                check_distributor_onchain_matches(
                    &account,
                    &merkle_tree,
                    new_distributor_args,
                    keypair.pubkey(),
                ).expect("merkle root on-chain does not match provided arguments! Confirm admin and clawback parameters to avoid loss of funds!");
            }
        }
    }
}

fn process_clawback(args: &Args, clawback_args: &ClawbackArgs) {
    let payer_keypair = read_keypair_file(&args.keypair_path).expect("Failed reading keypair file");
    let clawback_receiver_token_account = args
        .clawback_receiver_token_account
        .unwrap_or(payer_keypair.pubkey());
    let clawback_ata = get_associated_token_address(&clawback_receiver_token_account, &args.mint);

    let client = RpcClient::new_with_commitment(&args.rpc_url, CommitmentConfig::confirmed());

    let from = get_associated_token_address(&clawback_args.distributor, &args.mint);
    println!("from: {from}");
    println!("distributor: {}", clawback_args.distributor);
    println!("from: {}", from);
    println!("to: {}", clawback_ata);
    println!("payer: {}", payer_keypair.pubkey());

    let clawback_ix = Instruction {
        program_id: args.program_id,
        accounts: merkle_distributor::accounts::Clawback {
            distributor: clawback_args.distributor,
            from,
            to: clawback_ata,
            admin: payer_keypair.pubkey(),
            system_program: solana_program::system_program::ID,
            token_program: token::ID,
        }
        .to_account_metas(None),
        data: merkle_distributor::instruction::Clawback {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[clawback_ix],
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair],
        client.get_latest_blockhash().unwrap(),
    );

    let signature = client
        .send_and_confirm_transaction_with_spinner(&tx)
        .unwrap();

    println!("Successfully clawed back funds! signature: {signature:#?}");
}

fn process_create_merkle_tree(merkle_tree_args: &CreateMerkleTreeArgs) {
    let merkle_tree = AirdropMerkleTree::new_from_csv(&merkle_tree_args.csv_path).unwrap();
    merkle_tree.write_to_file(&merkle_tree_args.merkle_tree_path);
}

fn process_set_admin(args: &Args, set_admin_args: &SetAdminArgs) {
    let keypair = read_keypair_file(&args.keypair_path).expect("Failed reading keypair file");

    let client = RpcClient::new_with_commitment(&args.rpc_url, CommitmentConfig::confirmed());

    let (distributor, _bump) = get_merkle_distributor_pda(
        &args.program_id,
        &args.mint,
        &set_admin_args.distributor,
        args.airdrop_version,
    );

    let set_admin_ix = Instruction {
        program_id: args.program_id,
        accounts: merkle_distributor::accounts::SetAdmin {
            distributor,
            admin: keypair.pubkey(),
            new_admin: set_admin_args.new_admin,
        }
        .to_account_metas(None),
        data: merkle_distributor::instruction::SetAdmin {}.data(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[set_admin_ix],
        Some(&keypair.pubkey()),
        &[&keypair],
        client.get_latest_blockhash().unwrap(),
    );

    let signature = client
        .send_and_confirm_transaction_with_spinner(&tx)
        .unwrap();

    println!("Successfully set admin! signature: {signature:#?}");
}

fn process_distributor_pda(args: &Args) {
    let keypair = read_keypair_file(&args.keypair_path).expect("Failed reading keypair file");
    let (distributor_pubkey, _bump) = get_merkle_distributor_pda(
        &args.program_id,
        &args.mint,
        &keypair.pubkey(),
        args.airdrop_version,
    );
    println!("{distributor_pubkey}");
}
