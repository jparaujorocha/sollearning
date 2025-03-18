use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;
use crate::states::student::StudentInfo;
use crate::states::course::{Course, CourseCompletion};
use crate::constants::*;
use crate::error::SolLearningError;

#[derive(Accounts)]
#[instruction(amount: u64, course_id: String)]
pub struct MintToStudent<'info> {
    #[account(mut)]
    pub educator: Account<'info, EducatorAccount>,

    #[account(mut)]
    pub educator_authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    /// CHECK: This is the token mint that will be used to mint tokens to the student
    #[account(mut)]
    pub token_mint: AccountInfo<'info>,

    #[account(mut)]
    pub student_info: Account<'info, StudentInfo>,

    /// CHECK: This is the token account of the student that will receive the tokens
    pub student: AccountInfo<'info>,

    /// CHECK: This is the token account of the student that will receive the tokens
    #[account(mut)]
    pub student_token_account: AccountInfo<'info>,

    #[account(
        seeds = [COURSE_SEED, educator.key().as_ref(), course_id.as_bytes()],
        bump = course.bump,
        constraint = course.educator == educator.key() @ SolLearningError::CourseNotOwnedByEducator,
    )]
    pub course: Account<'info, Course>, 
    
    #[account(
        init,
        payer = educator_authority,
        space = 8 + std::mem::size_of::<CourseCompletion>(),
        seeds = [COURSE_COMPLETION_SEED, student.key().as_ref(), course_id.as_bytes()],
        bump,
        constraint = course.is_active @ SolLearningError::CourseInactive,
    )]
    pub course_completion: Account<'info, CourseCompletion>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}