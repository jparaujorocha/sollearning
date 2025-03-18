use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;
use crate::constants::EDUCATOR_SEED;

#[derive(Accounts)]
#[instruction(mint_limit: u64)]
pub struct RegisterEducator<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    /// CHECK: This is the token account of the student that will receive the tokens
    pub educator: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<EducatorAccount>(),
        seeds = [EDUCATOR_SEED, educator.key().as_ref()],
        bump,
    )]
    pub educator_account: Account<'info, EducatorAccount>,

    pub system_program: Program<'info, System>,
}
