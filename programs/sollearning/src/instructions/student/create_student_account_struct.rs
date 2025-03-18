use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::associated_token::AssociatedToken;
use crate::states::program::ProgramState;

#[derive(Accounts)]
pub struct CreateStudentTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    /// CHECK: `token_mint` is mutable
    #[account(address = program_state.token_mint)]
    pub token_mint: AccountInfo<'info>,

    /// CHECK: `student` is checked
    pub student: UncheckedAccount<'info>,

    /// CHECK: `student_token_account` is checked
    #[account(mut)]
    pub student_token_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
