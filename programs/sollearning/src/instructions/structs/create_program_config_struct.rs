use anchor_lang::prelude::*;
use crate::states::config::ProgramConfig;
use crate::constants::CONFIG_SEED;

#[derive(Accounts)]
pub struct CreateProgramConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<ProgramConfig>(),
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}