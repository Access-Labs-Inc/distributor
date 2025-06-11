//! A program for distributing tokens efficiently via uploading a [Merkle root](https://en.wikipedia.org/wiki/Merkle_tree).
//!
//! This program is largely based off of [Uniswap's Merkle Distributor](https://github.com/Uniswap/merkle-distributor).
//!
//! # Rationale
//!
//! Although Solana has low fees for executing transactions, it requires staking tokens to pay for storage costs, also known as "rent". These rent costs can add up when sending tokens to thousands or tens of thousands of wallets, making it economically unreasonable to distribute tokens to everyone.
//!
//! The Merkle distributor, pioneered by [Uniswap](https://github.com/Uniswap/merkle-distributor), solves this issue by deriving a 256-bit "root hash" from a tree of balances. This puts the gas cost on the claimer. Solana has the additional advantage of being able to reclaim rent from closed token accounts, so the net cost to the user should be around `0.000010 SOL` (at the time of writing).
//!
//! The Merkle distributor is also significantly easier to manage from an operations perspective, since one does not need to send a transaction to each individual address that may be redeeming tokens.

#![allow(clippy::too_many_arguments)]
use anchor_lang::prelude::*;
use instructions::*;
use solana_security_txt::security_txt;

pub mod error;
pub mod instructions;
pub mod state;

security_txt! {
    // Required fields
    name: "Merkle Distributor",
    project_url: "https://accessprotocol.co",
    contacts: "email:support@accessprotocol.co",
    policy: "https://github.com/Access-Labs-Inc/distributor",
    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/Access-Labs-Inc/distributor"
}

declare_id!("ADis3cccJHS6dmj8MPCG1NGGuXBN2ZSaaXFwKKU7UUgF");

#[program]
pub mod merkle_distributor {
    use super::*;

    #[allow(clippy::result_large_err)]
    pub fn new_distributor(
        ctx: Context<NewDistributor>,
        version: u64,
        root: [u8; 32],
        max_total_claim: u64,
        max_num_nodes: u64,
        start_vesting_ts: i64,
        end_vesting_ts: i64,
    ) -> Result<()> {
        handle_new_distributor(
            ctx,
            version,
            root,
            max_total_claim,
            max_num_nodes,
            start_vesting_ts,
            end_vesting_ts,
        )
    }

    #[allow(clippy::result_large_err)]
    pub fn new_claim(
        ctx: Context<NewClaim>,
        amount_unlocked: u64,
        amount_locked: u64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        handle_new_claim(ctx, amount_unlocked, amount_locked, proof)
    }

    #[allow(clippy::result_large_err)]
    pub fn claim_locked(ctx: Context<ClaimLocked>) -> Result<()> {
        handle_claim_locked(ctx)
    }

    #[allow(clippy::result_large_err)]
    pub fn clawback(ctx: Context<Clawback>) -> Result<()> {
        handle_clawback(ctx)
    }

    #[allow(clippy::result_large_err)]
    pub fn set_admin(ctx: Context<SetAdmin>) -> Result<()> {
        handle_set_admin(ctx)
    }
}
