use anchor_lang::prelude::*;
use anchor_spl::token::{self};
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;
use crate::states::student::StudentInfo;
use crate::states::course::{CourseCompletion, CourseCompleted, TokensMinted};
use crate::error::SolLearningError;
use crate::constants::*;
use crate::instructions::token::mint_struct::MintToStudent;
use crate::utils::pause::{check_program_running, check_function_running};

pub fn mint_to_student_handler(ctx: Context<MintToStudent>, amount: u64, course_id: String) -> Result<()> {
    check_program_running(&ctx.accounts.program_state)?;
    check_function_running(&ctx.accounts.program_state, PAUSE_FLAG_MINT)?;
    
    validate_mint_amount(amount, &ctx.accounts.educator)?;

    let current_time = Clock::get()?.unix_timestamp;
    
    validate_mint_cooldown(&ctx.accounts.educator, current_time)?;
    
    let student_previous_balance = token::accessor::amount(&ctx.accounts.student_token_account)?;

    let student_key = ctx.accounts.student.key();
    let educator_key = ctx.accounts.educator.key();
    let course_name = ctx.accounts.course.course_name.clone();

    {
        let educator = &mut ctx.accounts.educator;
        let program_state = &mut ctx.accounts.program_state;
        let student_info = &mut ctx.accounts.student_info;
        let course_completion = &mut ctx.accounts.course_completion;

        update_educator_stats(educator, amount, current_time)?;
        update_program_state(program_state, amount)?;
        update_student_info(student_info, amount, current_time)?;
        initialize_course_completion(course_completion, student_key, &course_id, educator_key, amount, current_time)?;
    }

    mint_tokens_to_student(&ctx, amount)?;
    emit_events(&ctx, amount, &course_id, current_time)?;
    log_minting(&ctx, amount, &course_id, student_previous_balance, &course_name)?;

    Ok(())
}

fn validate_mint_amount(amount: u64, educator: &Account<EducatorAccount>) -> Result<()> {
    require!(amount > 0 && amount <= educator.mint_limit, SolLearningError::InvalidAmount);
    Ok(())
}

fn validate_mint_cooldown(educator: &Account<EducatorAccount>, current_time: i64) -> Result<()> {
    let time_since_last_mint = current_time - educator.last_mint_time;
    require!(
        time_since_last_mint >= MINT_COOLDOWN_PERIOD || educator.last_mint_time == 0,
        SolLearningError::MintingTooFrequent
    );
    Ok(())
}

fn update_educator_stats(educator: &mut Account<EducatorAccount>, amount: u64, current_time: i64) -> Result<()> {
    educator.total_minted = educator.total_minted.checked_add(amount).ok_or(SolLearningError::Overflow)?;
    educator.last_mint_time = current_time;
    Ok(())
}

fn update_program_state(program_state: &mut Account<ProgramState>, amount: u64) -> Result<()> {
    program_state.total_minted = program_state.total_minted.checked_add(amount).ok_or(SolLearningError::Overflow)?;
    Ok(())
}

fn update_student_info(student_info: &mut Account<StudentInfo>, amount: u64, current_time: i64) -> Result<()> {
    student_info.total_earned = student_info.total_earned.checked_add(amount).ok_or(SolLearningError::Overflow)?;
    student_info.courses_completed = student_info.courses_completed.checked_add(1).ok_or(SolLearningError::Overflow)?;
    student_info.last_activity = current_time;
    Ok(())
}

fn initialize_course_completion(
    course_completion: &mut Account<CourseCompletion>,
    student: Pubkey,
    course_id: &str,
    educator: Pubkey,
    amount: u64,
    current_time: i64,
) -> Result<()> {
    course_completion.student = student;
    course_completion.course_id = course_id.to_string();
    course_completion.verified_by = educator;
    course_completion.completion_time = current_time;
    course_completion.tokens_awarded = amount;
    Ok(())
}

fn mint_tokens_to_student(ctx: &Context<MintToStudent>, amount: u64) -> Result<()> {
    let signer_seeds = &[PROGRAM_STATE_SEED, &[ctx.accounts.program_state.bump]];
    let signer = &[&signer_seeds[..]];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.student_token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            signer,
        ),
        amount,
    )?;
    Ok(())
}

fn emit_events(ctx: &Context<MintToStudent>, amount: u64, course_id: &str, timestamp: i64) -> Result<()> {
    emit!(CourseCompleted {
        student: ctx.accounts.student.key(),
        course_id: course_id.to_string(),
        educator: ctx.accounts.educator.key(),
        tokens_awarded: amount,
        timestamp,
    });

    emit!(TokensMinted {
        recipient: ctx.accounts.student.key(),
        amount,
        minted_by: ctx.accounts.educator_authority.key(),
        timestamp,
    });

    Ok(())
}

fn log_minting(ctx: &Context<MintToStudent>, amount: u64, course_id: &str, student_previous_balance: u64, course_name: &str) -> Result<()> {
    let student_new_balance = student_previous_balance + amount;
    msg!(
        "Minted {} tokens to student {} for completing course '{}' ({}) - Previous balance: {}, New balance: {}",
        amount,
        ctx.accounts.student.key(),
        course_name,
        course_id,
        student_previous_balance,
        student_new_balance
    );
    Ok(())
}