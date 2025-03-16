use anchor_lang::prelude::*;
use crate::states::educator::EducatorAccount;
use crate::constants::*;
use crate::error::SolLearningError;
use crate::instructions::structs::register_educator_struct::RegisterEducator;

pub fn register_educator_handler(ctx: Context<RegisterEducator>, mint_limit: u64) -> Result<()> {
    validate_mint_limit(mint_limit)?;

    // Access the bump directly
    let bump = ctx.bumps.educator_account;

    // Perform all mutable updates first
    _ = { 
        let educator_account: &mut Account<'_, EducatorAccount> = &mut ctx.accounts.educator_account;
        educator_account.educator_address = ctx.accounts.educator.key();
        educator_account.authority = ctx.accounts.authority.key();
        educator_account.mint_limit = mint_limit;
        educator_account.total_minted = 0;
        educator_account.is_active = true;
        educator_account.bump = bump;
    };

    // Log after mutable borrow ends
    log_registration(&ctx, mint_limit);

    Ok(())
}

fn validate_mint_limit(mint_limit: u64) -> Result<()> {
    require!(mint_limit > 0 && mint_limit <= MAX_MINT_AMOUNT, SolLearningError::InvalidAmount);
    Ok(())
}

fn log_registration(ctx: &Context<RegisterEducator>, mint_limit: u64) {
    msg!(
        "Registered educator {} with mint limit of {}",
        ctx.accounts.educator.key(),
        mint_limit
    );
}
