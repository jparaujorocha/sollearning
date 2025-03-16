use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use crate::states::program::ProgramState;
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BurnInstruction<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: `token_account` is mutable
    #[account(mut)]
    pub token_account: AccountInfo<'info>,

    /// CHECK: `token_mint` is mutable
    #[account(mut)]
    pub token_mint: AccountInfo<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}
