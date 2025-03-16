use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;
use crate::states::course::{Course, CourseHistory, CourseUpdated};use crate::constants::*;
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

    #[account(
        init,
        payer = educator_authority,
        space = 8 + 50 + 100 + 8 + 1 + 32 + 8 + 8 + 4 + MAX_CHANGE_DESCRIPTION_LENGTH + 1,
        seeds = [COURSE_HISTORY_SEED, course_id.as_bytes()],
        bump,
    )]
    pub course_history: Account<'info, CourseHistory>,

    pub system_program: Program<'info, System>,
}

pub fn update_course_handler(
    ctx: Context<UpdateCourse>,
    course_name: Option<String>,
    reward_amount: Option<u64>,
    is_active: Option<bool>,
    metadata_hash: Option<[u8; 32]>,
    change_description: String,
) -> Result<()> {
    require!(change_description.len() <= MAX_CHANGE_DESCRIPTION_LENGTH, 
        SolLearningError::DescriptionTooLong);

    let course = &mut ctx.accounts.course;
    let timestamp = Clock::get()?.unix_timestamp;

    // ✅ Agora usando `.clone()` para evitar o erro de valor movido
    let previous_name = course.course_name.clone();
    let previous_reward = course.reward_amount;
    let previous_status = course.is_active;
    let previous_metadata_hash = course.metadata_hash;

    let mut new_name = None;
    let mut new_reward = None;
    let mut new_active = None;

    if let Some(name) = course_name.clone() { // ✅ Usar `clone()` evita erro de valor movido
        require!(name.len() <= MAX_COURSE_NAME_LENGTH, SolLearningError::CourseNameTooLong);
        course.course_name = name.clone();
        new_name = Some(name);
    }

    if let Some(reward) = reward_amount {
        require!(reward > 0, SolLearningError::InvalidCourseReward);
        course.reward_amount = reward;
        new_reward = Some(reward);
    }

    if let Some(active) = is_active {
        course.is_active = active;
        new_active = Some(active);
    }

    if let Some(hash) = metadata_hash {
        course.metadata_hash = hash;
    }

    course.version = course.version.checked_add(1).ok_or(SolLearningError::Overflow)?;
    course.last_updated_at = timestamp;

    // ✅ Agora usando `.clone()` para evitar erro de valor movido na emissão do evento
    let course_history = &mut ctx.accounts.course_history;
    course_history.course_id = course.course_id.clone();
    course_history.educator = ctx.accounts.educator.key();
    course_history.version = course.version;
    course_history.previous_name = previous_name.clone();
    course_history.previous_reward = previous_reward;
    course_history.previous_active = previous_status;
    course_history.previous_metadata_hash = previous_metadata_hash;
    course_history.updated_by = ctx.accounts.educator_authority.key();
    course_history.updated_at = timestamp;
    course_history.change_description = change_description.clone();
    course_history.bump = ctx.bumps.course_history;

    // ✅ Corrigindo a emissão do evento para não mover valores
    emit!(CourseUpdated {
        course_id: course.course_id.clone(),
        educator: ctx.accounts.educator.key(),
        version: course.version,
        previous_name: previous_name.clone(),
        new_name,
        previous_reward,
        new_reward,
        previous_active: previous_status,
        new_active,
        updated_by: ctx.accounts.educator.key(),
        timestamp,
    });

    msg!("Course {} updated by {}", course.course_id, ctx.accounts.educator_authority.key());

    Ok(())
}
