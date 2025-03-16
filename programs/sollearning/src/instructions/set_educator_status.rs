use anchor_lang::prelude::*;
use crate::states::educator::EducatorAccount;
use crate::constants::*;
use crate::error::SolLearningError;
use crate::instructions::structs::set_educator_status_struct::SetEducatorStatus;

pub fn set_educator_status_handler(
    ctx: Context<SetEducatorStatus>,
    is_active: bool,
    new_mint_limit: Option<u64>,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;

    update_educator_status(&mut ctx.accounts.educator_account, is_active, new_mint_limit, current_time)?;

    log_status_update(&ctx, is_active, new_mint_limit);

    Ok(())
}

fn update_educator_status(
    educator_account: &mut Account<EducatorAccount>,
    is_active: bool,
    new_mint_limit: Option<u64>,
    current_time: i64,
) -> Result<()> {
    educator_account.is_active = is_active;

    if let Some(mint_limit) = new_mint_limit {
        require!(mint_limit > 0 && mint_limit <= MAX_MINT_AMOUNT, SolLearningError::InvalidAmount);
        educator_account.mint_limit = mint_limit;
    }

    educator_account.last_updated_at = current_time;
    Ok(())
}

fn log_status_update(ctx: &Context<SetEducatorStatus>, is_active: bool, new_mint_limit: Option<u64>) {
    let mint_limit_msg = match new_mint_limit {
        Some(limit) => format!(", new mint limit: {}", limit),
        None => String::new(),
    };

    msg!(
        "Updated educator {} status to {}{}",
        ctx.accounts.educator.key(),
        if is_active { "active" } else { "inactive" },
        mint_limit_msg
    );
}
