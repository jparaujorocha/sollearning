use anchor_lang::prelude::*;
use crate::states::educator::EducatorAccount;
use crate::states::course::{Course, CourseCreated};
use crate::constants::*;
use crate::error::SolLearningError;
use crate::instructions::structs::create_course_struct::CreateCourse;
use crate::utils::pause::{check_program_running, check_function_running};

pub fn create_course_handler(
    ctx: Context<CreateCourse>,
    course_id: String,
    course_name: String,
    reward_amount: u64,
    metadata_hash: [u8; 32],
) -> Result<()> {
    check_program_running(&ctx.accounts.program_state)?;
    check_function_running(&ctx.accounts.program_state, PAUSE_FLAG_COURSE)?;

    validate_course_details(&course_id, &course_name, reward_amount, &ctx.accounts.educator)?;

    let current_time = Clock::get()?.unix_timestamp;

    initialize_course(
        &mut ctx.accounts.course,
        &course_id,
        &course_name,
        ctx.accounts.educator.key(),
        reward_amount,
        metadata_hash,
        current_time,
        ctx.bumps.course,
    )?;

    increment_educator_course_count(&mut ctx.accounts.educator)?;

    emit_course_created(&course_id, &course_name, ctx.accounts.educator.key(), reward_amount, current_time)?;

    msg!(
        "Created course '{}' with ID {} by educator {}",
        course_name,
        course_id,
        ctx.accounts.educator.key()
    );

    Ok(())
}

fn validate_course_details(course_id: &str, course_name: &str, reward_amount: u64, educator: &Account<EducatorAccount>) -> Result<()> {
    require!(
        !course_id.is_empty() && course_id.len() <= MAX_COURSE_ID_LENGTH,
        SolLearningError::CourseIdTooLong
    );

    require!(
        !course_name.is_empty() && course_name.len() <= MAX_COURSE_NAME_LENGTH,
        SolLearningError::CourseNameTooLong
    );

    require!(
        reward_amount > 0 && reward_amount <= educator.mint_limit,
        SolLearningError::InvalidCourseReward
    );

    Ok(())
}

fn initialize_course(
    course: &mut Account<Course>,
    course_id: &str,
    course_name: &str,
    educator_key: Pubkey,
    reward_amount: u64,
    metadata_hash: [u8; 32],
    current_time: i64,
    bump: u8,
) -> Result<()> {
    course.course_id = course_id.to_string();
    course.course_name = course_name.to_string();
    course.educator = educator_key;
    course.reward_amount = reward_amount;
    course.completion_count = 0;
    course.is_active = true;
    course.metadata_hash = metadata_hash;
    course.created_at = current_time;
    course.last_updated_at = current_time;
    course.bump = bump;
    Ok(())
}

fn increment_educator_course_count(educator: &mut Account<EducatorAccount>) -> Result<()> {
    educator.course_count = educator
        .course_count
        .checked_add(1)
        .ok_or(SolLearningError::Overflow)?;
    Ok(())
}

fn emit_course_created(course_id: &str, course_name: &str, educator_key: Pubkey, reward_amount: u64, timestamp: i64) -> Result<()> {
    emit!(CourseCreated {
        course_id: course_id.to_string(),
        course_name: course_name.to_string(),
        educator: educator_key,
        reward_amount,
        timestamp,
    });
    Ok(())
}