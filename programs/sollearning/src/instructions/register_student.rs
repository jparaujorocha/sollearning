use anchor_lang::prelude::*;
use crate::state::{ProgramState, StudentInfo, StudentRegistered};
use crate::constants::*;
use crate::error::SolLearningError;

#[derive(Accounts)]
pub struct RegisterStudent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub student: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<StudentInfo>(),
        seeds = [STUDENT_SEED, student.key().as_ref()],
        bump,
    )]
    pub student_info: Account<'info, StudentInfo>,

    pub system_program: Program<'info, System>,
}

pub fn register_student_handler(ctx: Context<RegisterStudent>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Encontre o bump manualmente
    let (_, bump) = Pubkey::find_program_address(
        &[STUDENT_SEED, ctx.accounts.student.key().as_ref()],
        ctx.program_id
    );
    
    let student_info = &mut ctx.accounts.student_info;
    student_info.student_address = ctx.accounts.student.key();
    student_info.total_earned = 0;
    student_info.courses_completed = 0;
    student_info.last_activity = current_time;
    student_info.bump = bump;

    // Emit event for registration
    emit!(StudentRegistered {
        student: ctx.accounts.student.key(),
        timestamp: current_time,
    });

    msg!(
        "Registered student {} (first-time registration)",
        ctx.accounts.student.key()
    );

    Ok(())
}