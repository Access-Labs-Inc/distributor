use crate::error::ErrorCode;
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};
use solana_program::system_instruction;

/// Accounts for [merkle_distributor::handle_new_distributor].
#[derive(Accounts)]
#[instruction(amount_lamports: u64)]
pub struct WithdrawCustodySol<'info> {
    /// [SolCustody].\
    /// CHECK: The sol_custody doesn't even need to exist, it's just a bucket for SOL.
    #[account(        
        mut,
        seeds = [
            b"SolCustody".as_ref(),
            owner.key().to_bytes().as_ref(),
        ],
        bump,
    )]
    pub sol_custody: AccountInfo<'info>,

    /// The owner of the sol custody.
    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[allow(clippy::result_large_err)]
pub fn handle_withdraw_custody_sol(
    ctx: Context<WithdrawCustodySol>,
    amount_lamports: u64,
) -> Result<()> {
    let sol_custody_lamports = ctx.accounts.sol_custody.to_account_info().lamports();
    require_gte!(
        sol_custody_lamports,
        amount_lamports,
        ErrorCode::InsufficientFunds
    );

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(ctx.accounts.sol_custody.key, ctx.accounts.owner.key, amount_lamports);



    // claim rewards
    let bump_seed = [*ctx.bumps.get("sol_custody").unwrap()];
    let signer_seeds: &[&[&[u8]]] = &[&[
        "SolCustody".as_bytes(),
        ctx.accounts.owner.key.as_ref(),
        &bump_seed.as_ref(),
    ]];

    // Invoke the transfer instruction
    
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.sol_custody.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &signer_seeds
    )?;


    Ok(())
}
