use anchor_lang::prelude::*;
use crate::state::ProgramState;
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
pub struct EmergencyToggle<'info> {
    #[account(
        mut,
        constraint = authority.key() == program_state.authority @ SolLearningError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
    )]
    pub program_state: Account<'info, ProgramState>,
}

pub fn emergency_toggle_handler(
    ctx: Context<EmergencyToggle>,
    paused: bool,
) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    program_state.paused = paused;

    msg!(
        "Program {} by authority {}",
        if paused { "paused" } else { "resumed" },
        ctx.accounts.authority.key()
    );

    Ok(())
}
