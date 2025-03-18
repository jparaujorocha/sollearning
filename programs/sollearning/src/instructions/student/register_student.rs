use anchor_lang::prelude::*;
use crate::states::student::{StudentInfo, StudentRegistered};
use crate::instructions::student::register_student_struct::RegisterStudent;


pub fn register_student_handler(ctx: Context<RegisterStudent>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;

    _ = initialize_student_info(&mut ctx.accounts.student_info, &ctx.accounts.student, current_time);

    emit_registration_event(&ctx, current_time);

    log_registration(&ctx);

    Ok(())
}

fn initialize_student_info(
    student_info: &mut Account<StudentInfo>,
    student: &Signer,
    current_time: i64,
) {
    student_info.student_address = student.key();
    student_info.total_earned = 0;
    student_info.courses_completed = 0;
    student_info.last_activity = current_time;
    student_info.bump = student_info.bump;
}

fn emit_registration_event(ctx: &Context<RegisterStudent>, timestamp: i64) {
    emit!(StudentRegistered {
        student: ctx.accounts.student.key(),
        timestamp,
    });
}

fn log_registration(ctx: &Context<RegisterStudent>) {
    msg!(
        "Registered student {} (first-time registration)",
        ctx.accounts.student.key()
    );
}
