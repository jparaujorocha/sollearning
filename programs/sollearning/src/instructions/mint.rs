use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};
use crate::{
    state::{ProgramState, EducatorAccount, StudentInfo},
    error::SolLearningError,
};

#[derive(Accounts)]
#[instruction(amount: u64, course_id: String)]
pub struct MintToStudent<'info> {
    #[account(
        mut,
        constraint = educator.is_active @ SolLearningError::Unauthorized,
    )]
    pub educator: Account<'info, EducatorAccount>,
    
    #[account(
        mut,
        constraint = educator_authority.key() == educator.educator_address @ SolLearningError::Unauthorized,
    )]
    pub educator_authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"program-state".as_ref()],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        mut,
        address = program_state.token_mint @ SolLearningError::InvalidMint,
    )]
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [crate::constants::STUDENT_SEED, student.key().as_ref()],
        bump,
    )]
    pub student_info: Account<'info, StudentInfo>,
    
    /// CHECK: This is the student who will receive tokens
    pub student: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = student_token_account.mint == token_mint.key() @ SolLearningError::InvalidMint,
        constraint = student_token_account.owner == student.key() @ SolLearningError::Unauthorized,
    )]
    pub student_token_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_to_student_handler(
    ctx: Context<MintToStudent>,
    amount: u64,
    course_id: String,
) -> Result<()> {
    // Validate amount
    require!(
        amount > 0 && amount <= ctx.accounts.educator.mint_limit,
        SolLearningError::InvalidAmount
    );
    
    // Update educator stats
    let educator = &mut ctx.accounts.educator;
    educator.total_minted = educator
        .total_minted
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;
    
    // Update program state
    let program_state = &mut ctx.accounts.program_state;
    program_state.total_minted = program_state
        .total_minted
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;
    
    // Update student info
    let student_info = &mut ctx.accounts.student_info;
    student_info.total_earned = student_info
        .total_earned
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;
    student_info.courses_completed = student_info
        .courses_completed
        .checked_add(1)
        .ok_or(SolLearningError::Overflow)?;
    student_info.last_activity = Clock::get()?.unix_timestamp;
    
    // Mint tokens to student
    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.student_token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            &[&[
                b"program-state".as_ref(),
                &[ctx.bumps.program_state],
            ]],
        ),
        amount,
    )?;
    
    msg!(
        "Minted {} tokens to student {} for completing course {}",
        amount,
        ctx.accounts.student.key(),
        course_id
    );
    
    Ok(())
}