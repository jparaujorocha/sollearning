use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;
use crate::states::course::{Course, CourseHistory};
use crate::constants::*;

#[derive(Accounts)]
#[instruction(course_id: String)]
pub struct UpdateCourse<'info> {
    #[account(mut)]
    pub educator: Account<'info, EducatorAccount>,

    #[account(mut)]
    pub educator_authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub course: Account<'info, Course>,

    #[account(
        init,
        payer = educator_authority,
        space = calculate_course_history_space(), 
        seeds = [COURSE_HISTORY_SEED, course_id.as_bytes()],
        bump,
    )]
    pub course_history: Account<'info, CourseHistory>,

    pub system_program: Program<'info, System>,
}

pub fn calculate_course_history_space() -> usize {
    8 + 50 + 100 + 8 + 1 + 32 + 8 + 8 + 4 + MAX_CHANGE_DESCRIPTION_LENGTH + 1
}
