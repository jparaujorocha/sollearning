use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;

#[derive(Accounts)]
pub struct SetEducatorStatus<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    /// CHECK: This is the token account of the student that will receive the tokens
    pub educator: AccountInfo<'info>,

    #[account(mut)]
    pub educator_account: Account<'info, EducatorAccount>,
}
