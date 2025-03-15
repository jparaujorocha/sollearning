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
) -> Result<()> {
    let educator_account = &mut ctx.accounts.educator_account;
    educator_account.is_active = is_active;

    msg!(
        "Updated educator {} status to {}",
        ctx.accounts.educator.key(),
        if is_active { "active" } else { "inactive" }
    );

    Ok(())
}
