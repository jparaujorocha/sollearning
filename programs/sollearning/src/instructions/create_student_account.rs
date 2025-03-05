use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};
use crate::{
    state::ProgramState,
    error::SolLearningError,
};

#[derive(Accounts)]
pub struct CreateStudentTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        seeds = [b"program-state".as_ref()],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        address = program_state.token_mint @ SolLearningError::InvalidMint,
    )]
    pub token_mint: Account<'info, Mint>,
    
    /// CHECK: This is the student for whom we're creating a token account
    pub student: AccountInfo<'info>,
    
    #[account(
        init,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = student,
    )]
    pub student_token_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_student_token_account_handler(ctx: Context<CreateStudentTokenAccount>) -> Result<()> {
    msg!(
        "Created token account for student: {}",
        ctx.accounts.student.key()
    );
    
    Ok(())
}