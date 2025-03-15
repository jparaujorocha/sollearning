use anchor_lang::prelude::*;
use crate::state::{ProgramState, EducatorAccount};
use crate::constants::*;
use crate::error::SolLearningError;

#[derive(Accounts)]
pub struct SetEducatorStatus<'info> {
    #[account(
        mut,
        constraint = authority.key() == program_state.authority @ SolLearningError::Unauthorized,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub authority: Signer<'info>,

    #[account(
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
    )]
    pub program_state: Account<'info, ProgramState>,
    /// CHECK: Used only as a reference to derive the PDA of the educator account
    pub educator: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [EDUCATOR_SEED, educator.key().as_ref()],
        bump = educator_account.bump,
    )]
    pub educator_account: Account<'info, EducatorAccount>,
}

pub fn set_educator_status_handler(
    ctx: Context<SetEducatorStatus>,
    is_active: bool,
    new_mint_limit: Option<u64>,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let educator_account = &mut ctx.accounts.educator_account;
    
    // Update status
    educator_account.is_active = is_active;
    
    // Update mint limit if provided
    if let Some(mint_limit) = new_mint_limit {
        require!(mint_limit > 0 && mint_limit <= MAX_MINT_AMOUNT, SolLearningError::InvalidAmount);
        educator_account.mint_limit = mint_limit;
    }
    
    // Update last updated timestamp
    educator_account.last_updated_at = current_time;

    msg!(
        "Updated educator {} status to {}{}",
        ctx.accounts.educator.key(),
        if is_active { "active" } else { "inactive" },
        if new_mint_limit.is_some() { format!(", new mint limit: {}", new_mint_limit.unwrap()) } else { String::from("") }
    );

    Ok(())
}