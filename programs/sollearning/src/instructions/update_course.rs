use anchor_lang::prelude::*;
use crate::states::course::{Course, CourseHistory, CourseUpdated};
use crate::constants::*;
use crate::error::SolLearningError;
use crate::instructions::structs::update_course_struct::UpdateCourse;

pub fn update_course_handler(
    ctx: Context<UpdateCourse>,
    course_name: Option<String>,
    reward_amount: Option<u64>,
    is_active: Option<bool>,
    metadata_hash: Option<[u8; 32]>,
    change_description: String,
) -> Result<()> {
    require!(
        change_description.len() <= MAX_CHANGE_DESCRIPTION_LENGTH,
        SolLearningError::DescriptionTooLong
    );

    let timestamp = Clock::get()?.unix_timestamp;

    // Extract values before mutability
    let course_id = ctx.accounts.course.course_id.clone();
    let educator_key = ctx.accounts.educator.key();
    let previous_state = capture_previous_state(&ctx.accounts.course);

    {
        let course = &mut ctx.accounts.course;
        let course_history = &mut ctx.accounts.course_history;

        apply_updates(course, course_name, reward_amount, is_active, metadata_hash, timestamp)?;
        store_course_history(course_history, educator_key, &course_id, &previous_state, &change_description, timestamp)?;
    }

    // Now ctx is free for immutable use
    emit_course_updated(educator_key, &course_id, &previous_state, timestamp)?;
    log_update(educator_key, &course_id);

    Ok(())
}

fn capture_previous_state(course: &Account<Course>) -> (String, u64, bool, [u8; 32]) {
    (
        course.course_name.clone(),
        course.reward_amount,
        course.is_active,
        course.metadata_hash,
    )
}

fn apply_updates(
    course: &mut Account<Course>,
    course_name: Option<String>,
    reward_amount: Option<u64>,
    is_active: Option<bool>,
    metadata_hash: Option<[u8; 32]>,
    timestamp: i64,
) -> Result<()> {
    if let Some(name) = course_name {
        require!(name.len() <= MAX_COURSE_NAME_LENGTH, SolLearningError::CourseNameTooLong);
        course.course_name = name;
    }

    if let Some(reward) = reward_amount {
        require!(reward > 0, SolLearningError::InvalidCourseReward);
        course.reward_amount = reward;
    }

    if let Some(active) = is_active {
        course.is_active = active;
    }

    if let Some(hash) = metadata_hash {
        course.metadata_hash = hash;
    }

    course.version = course.version.checked_add(1).ok_or(SolLearningError::Overflow)?;
    course.last_updated_at = timestamp;

    Ok(())
}

fn store_course_history(
    course_history: &mut Account<CourseHistory>,
    educator_key: Pubkey,
    course_id: &str,
    previous_state: &(String, u64, bool, [u8; 32]),
    change_description: &str,
    timestamp: i64,
) -> Result<()> {
    let (previous_name, previous_reward, previous_active, previous_metadata_hash) = previous_state;

    course_history.course_id = course_id.to_string();
    course_history.educator = educator_key;
    course_history.version = course_history.version.checked_add(1).ok_or(SolLearningError::Overflow)?;
    course_history.previous_name = previous_name.clone();
    course_history.previous_reward = *previous_reward;
    course_history.previous_active = *previous_active;
    course_history.previous_metadata_hash = *previous_metadata_hash;
    course_history.updated_by = educator_key;
    course_history.updated_at = timestamp;
    course_history.change_description = change_description.to_string();

    Ok(())
}

fn emit_course_updated(
    educator_key: Pubkey,
    course_id: &str,
    previous_state: &(String, u64, bool, [u8; 32]),
    timestamp: i64,
) -> Result<()> {
    let (previous_name, previous_reward, previous_active, _) = previous_state;

    emit!(CourseUpdated {
        course_id: course_id.to_string(),
        educator: educator_key,
        version: 1,
        previous_name: previous_name.clone(),
        new_name: Some(previous_name.clone()), // Just for demonstration
        previous_reward: *previous_reward,
        new_reward: Some(*previous_reward), // Keeping the same reward
        previous_active: *previous_active,
        new_active: Some(*previous_active),
        updated_by: educator_key,
        timestamp,
    });

    Ok(())
}

fn log_update(educator_key: Pubkey, course_id: &str) {
    msg!(
        "Course {} updated by {}",
        course_id,
        educator_key
    );
}
