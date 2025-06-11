// Instruction to clawback funds once they have expired

use anchor_lang::{context::Context, prelude::*, Accounts, Key, Result};
use anchor_spl::{
    token,
    token::{Token, TokenAccount},
};

use crate::{error::ErrorCode, state::merkle_distributor::MerkleDistributor};

/// [merkle_distributor::clawback] accounts.
#[derive(Accounts)]
pub struct Clawback<'info> {
    /// The [MerkleDistributor].
    #[account(mut)]
    pub distributor: Account<'info, MerkleDistributor>,

    /// Distributor ATA containing the tokens to distribute.
    #[account(
        mut,
        associated_token::mint = distributor.mint,
        associated_token::authority = distributor.key(),
        address = distributor.token_vault
    )]
    pub from: Account<'info, TokenAccount>,

    /// The Clawback token account.
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    /// Admin account
    /// Only admin can claw back
    #[account(mut, address = distributor.admin @ ErrorCode::Unauthorized)]
    pub admin: Signer<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,

    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}

/// Claws back unclaimed tokens by:
/// 1. Transferring remaining funds from the vault to the clawback receiver
/// 2. Marking the distributor as clawed back
#[allow(clippy::result_large_err)]
pub fn handle_clawback(ctx: Context<Clawback>) -> Result<()> {
    let distributor = &ctx.accounts.distributor;

    require!(!distributor.clawed_back, ErrorCode::ClawbackAlreadyClaimed);

    let seeds = [
        b"MerkleDistributor".as_ref(),
        &distributor.mint.to_bytes(),
        &distributor.creator.to_bytes(),
        &distributor.version.to_le_bytes(),
        &[ctx.accounts.distributor.bump],
    ];

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.distributor.to_account_info(),
            },
        )
        .with_signer(&[&seeds[..]]),
        ctx.accounts.from.amount,
    )?;

    let distributor = &mut ctx.accounts.distributor;

    distributor.clawed_back = true;

    Ok(())
}
