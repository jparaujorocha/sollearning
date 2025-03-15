use anchor_lang::prelude::*;
use crate::state::{ProgramState, EducatorAccount, Course, CourseCreated};
use crate::constants::*;
use crate::error::SolLearningError;

#[derive(Accounts)]
#[instruction(course_id: String, course_name: String, reward_amount: u64, metadata_hash: [u8; 32])]
pub struct CreateCourse<'info> {
    #[account(
        mut,
        constraint = educator.is_active @ SolLearningError::InactiveEducator,
        constraint = educator.course_count < MAX_COURSES_PER_EDUCATOR @ SolLearningError::MaxCoursesPerEducatorReached,
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
        init,
        payer = educator_authority,
        space = 8 + 4 + course_id.len() + 4 + course_name.len() + 32 + 8 + 4 + 1 + 32 + 8 + 8 + 1,
        seeds = [COURSE_SEED, educator.key().as_ref(), course_id.as_bytes()],
        bump,
    )]
    pub course: Account<'info, Course>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_course_handler(
    ctx: Context<CreateCourse>,
    course_id: String,
    course_name: String,
    reward_amount: u64,
    metadata_hash: [u8; 32],
) -> Result<()> {
    // Validate course_id and course_name lengths
    require!(!course_id.is_empty() && course_id.len() <= MAX_COURSE_ID_LENGTH, 
        SolLearningError::CourseIdTooLong);
    
    require!(!course_name.is_empty() && course_name.len() <= MAX_COURSE_NAME_LENGTH, 
        SolLearningError::CourseNameTooLong);
    
    // Validate reward amount is within educator's limit
    require!(reward_amount > 0 && reward_amount <= ctx.accounts.educator.mint_limit, 
        SolLearningError::InvalidCourseReward);
    
    let current_time = Clock::get()?.unix_timestamp;
    
    // Update the course account
    let course = &mut ctx.accounts.course;
    course.course_id = course_id.clone();
    course.course_name = course_name.clone();
    course.educator = ctx.accounts.educator.key();
    course.reward_amount = reward_amount;
    course.completion_count = 0;
    course.is_active = true;
    course.metadata_hash = metadata_hash;
    course.created_at = current_time;
    course.last_updated_at = current_time;
    course.bump = ctx.bumps.course;
    
    // Update educator's course count
    let educator = &mut ctx.accounts.educator;
    educator.course_count = educator.course_count.checked_add(1)
        .ok_or(SolLearningError::Overflow)?;
    
    // Emit course creation event
    emit!(CourseCreated {
        course_id: course_id.clone(),
        course_name: course_name.clone(),
        educator: ctx.accounts.educator.key(),
        reward_amount,
        timestamp: current_time,
    });
    
    msg!(
        "Created course '{}' with ID {} by educator {}",
        course_name,
        course_id,
        ctx.accounts.educator.key()
    );
    
    Ok(())
}