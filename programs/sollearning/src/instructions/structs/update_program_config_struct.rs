use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::config::ProgramConfig;
use crate::error::SolLearningError;

#[derive(Accounts)]
pub struct UpdateProgramConfig<'info> {
    #[account(
        mut,
        constraint = authority.key() == program_config.authority @ SolLearningError::UnauthorizedAuthority
    )]
    pub authority: Signer<'info>,

    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub program_config: Account<'info, ProgramConfig>,
}