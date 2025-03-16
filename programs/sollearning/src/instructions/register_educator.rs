use anchor_lang::prelude::*;
use crate::states::educator::EducatorAccount;
use crate::constants::*;
use crate::error::SolLearningError;
use crate::instructions::structs::register_educator_struct::RegisterEducator;
use crate::utils::pause::{check_program_running, check_function_running};

pub fn register_educator_handler(ctx: Context<RegisterEducator>, mint_limit: u64) -> Result<()> {
    check_program_running(&ctx.accounts.program_state)?;
    check_function_running(&ctx.accounts.program_state, PAUSE_FLAG_REGISTER)?;
    
    validate_mint_limit(mint_limit)?;

    let bump: u8 = ctx.bumps.educator_account;
    let current_time: i64 = Clock::get()?.unix_timestamp;

    _ = { 
        let educator_account: &mut Account<'_, EducatorAccount> = &mut ctx.accounts.educator_account;
        educator_account.educator_address = ctx.accounts.educator.key();
        educator_account.authority = ctx.accounts.authority.key();
        educator_account.mint_limit = mint_limit;
        educator_account.total_minted = 0;
        educator_account.is_active = true;
        educator_account.created_at = current_time;
        educator_account.last_updated_at = current_time;
        educator_account.last_mint_time = 0;
        educator_account.bump = bump;
    };

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