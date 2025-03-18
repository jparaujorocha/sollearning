use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;
use crate::states::course::Course;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(course_id: String, course_name: String, reward_amount: u64, metadata_hash: [u8; 32])]
pub struct CreateCourse<'info> {
    #[account(mut)]
    pub educator: Account<'info, EducatorAccount>,

    #[account(mut)]
    pub educator_authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = educator_authority,
        space = calculate_course_space(&course_id, &course_name),
        seeds = [COURSE_SEED, educator.key().as_ref(), course_id.as_bytes()],
        bump,
    )]
    pub course: Account<'info, Course>,

    pub system_program: Program<'info, System>,
}

pub fn calculate_course_space(course_id: &str, course_name: &str) -> usize {
    8 + 4 + course_id.len() + 4 + course_name.len() + 32 + 8 + 4 + 1 + 32 + 8 + 8 + 1
}
