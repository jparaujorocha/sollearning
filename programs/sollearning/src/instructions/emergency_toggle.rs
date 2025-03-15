use anchor_lang::prelude::*;
use crate::state::{ProgramState, ProgramStatusChanged};
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
    
    // Update program paused status
    program_state.paused = paused;
    
    // Get current timestamp for event
    let current_time = Clock::get()?.unix_timestamp;
    
    // Emit event for program status change
    emit!(ProgramStatusChanged {
        paused,
        authority: ctx.accounts.authority.key(),
        timestamp: current_time,
    });

    msg!(
        "Program {} by authority {}",
        if paused { "paused" } else { "resumed" },
        ctx.accounts.authority.key()
    );

    Ok(())
}