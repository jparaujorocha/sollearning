use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use crate::states::program::ProgramState;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferInstruction<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: checked
    #[account(mut)]
    pub from: AccountInfo<'info>,

    /// CHECK: checked
    #[account(mut)]
    pub to: AccountInfo<'info>,

    /// CHECK: checked
    #[account(mut)]
    pub token_mint: AccountInfo<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}
