use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{ProgramState, EducatorAccount, StudentInfo, CourseCompletion};
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(amount: u64, course_id: String)]
pub struct MintToStudent<'info> {
    #[account(
        constraint = educator.is_active @ SolLearningError::InactiveEducator,
    )]
    pub educator: Account<'info, EducatorAccount>,
    
    #[account(
        mut,
        constraint = educator_authority.key() == educator.educator_address @ SolLearningError::Unauthorized,
    )]
    pub educator_authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        mut,
        address = program_state.token_mint,
    )]
    pub token_mint: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [STUDENT_SEED, student.key().as_ref()],
        bump = student_info.bump,
    )]
    pub student_info: Account<'info, StudentInfo>,
    
    pub student: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = token::accessor::mint(&student_token_account)? == program_state.token_mint @ SolLearningError::InvalidMint,
        constraint = token::accessor::authority(&student_token_account)? == student.key() @ SolLearningError::Unauthorized,
    )]
    pub student_token_account: AccountInfo<'info>,
    
    #[account(
        init,
        payer = educator_authority,
        space = 8 + 32 + 4 + course_id.len() + 32 + 8 + 8 + 1,
        seeds = [COURSE_COMPLETION_SEED, student.key().as_ref(), course_id.as_bytes()],
        bump,
    )]
    pub course_completion: Account<'info, CourseCompletion>,
    
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
    require!(amount > 0 && amount <= ctx.accounts.educator.mint_limit, SolLearningError::InvalidAmount);
    require!(!course_id.is_empty() && course_id.len() <= 50, SolLearningError::InvalidCourseId);
    
    let educator = &mut ctx.accounts.educator;
    educator.total_minted = educator.total_minted.checked_add(amount).ok_or(SolLearningError::Overflow)?;
    
    let program_state = &mut ctx.accounts.program_state;
    program_state.total_minted = program_state.total_minted.checked_add(amount).ok_or(SolLearningError::Overflow)?;
    
    let student_info = &mut ctx.accounts.student_info;
    student_info.total_earned = student_info.total_earned.checked_add(amount).ok_or(SolLearningError::Overflow)?;
    student_info.courses_completed = student_info.courses_completed.checked_add(1).ok_or(SolLearningError::Overflow)?;
    student_info.last_activity = Clock::get()?.unix_timestamp;
    
    let course_completion = &mut ctx.accounts.course_completion;
    course_completion.student = ctx.accounts.student.key();
    course_completion.course_id = course_id.clone();
    course_completion.verified_by = ctx.accounts.educator_authority.key();
    course_completion.completion_time = Clock::get()?.unix_timestamp;
    course_completion.tokens_awarded = amount;
    course_completion.bump = ctx.bumps.course_completion;
    
    let program_state_seeds = &[PROGRAM_STATE_SEED, &[program_state.bump]];
    let program_state_signer = &[&program_state_seeds[..]];
    
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.student_token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            program_state_signer,
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
