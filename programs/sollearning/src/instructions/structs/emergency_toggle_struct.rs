use anchor_lang::prelude::*;
use crate::states::program::ProgramState;

#[derive(Accounts)]
pub struct EmergencyToggle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
}
