use anchor_lang::prelude::*;
use crate::state::{ProgramState, EducatorAccount, Course};
use crate::constants::*;
use crate::error::SolLearningError;

#[derive(Accounts)]
#[instruction(course_id: String)]
pub struct UpdateCourse<'info> {
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
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        mut,
        seeds = [COURSE_SEED, educator.key().as_ref(), course_id.as_bytes()],
        bump = course.bump,
        constraint = course.educator == educator.key() @ SolLearningError::CourseNotOwnedByEducator,
    )]
    pub course: Account<'info, Course>,
    
    pub system_program: Program<'info, System>,
}

pub fn update_course_handler(
    ctx: Context<UpdateCourse>,
    course_name: Option<String>,
    reward_amount: Option<u64>,
    is_active: Option<bool>,
    metadata_hash: Option<[u8; 32]>,
) -> Result<()> {
    let course = &mut ctx.accounts.course;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Update course name if provided
    if let Some(name) = course_name {
        require!(!name.is_empty() && name.len() <= MAX_COURSE_NAME_LENGTH, 
            SolLearningError::CourseNameTooLong);
        course.course_name = name;
    }
    
    // Update reward amount if provided
    if let Some(reward) = reward_amount {
        require!(reward > 0 && reward <= ctx.accounts.educator.mint_limit, 
            SolLearningError::InvalidCourseReward);
        course.reward_amount = reward;
    }
    
    // Update active status if provided
    if let Some(active) = is_active {
        course.is_active = active;
    }
    
    // Update metadata hash if provided
    if let Some(hash) = metadata_hash {
        course.metadata_hash = hash;
    }
    
    // Update last updated timestamp
    course.last_updated_at = current_time;
    
    msg!(
        "Updated course {} by educator {}",
        course.course_id,
        ctx.accounts.educator.key()
    );
    
    Ok(())
}