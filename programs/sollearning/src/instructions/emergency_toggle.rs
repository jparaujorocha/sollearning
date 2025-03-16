use anchor_lang::prelude::*;
use crate::states::program::{ProgramState, ProgramStatusChanged};
use crate::instructions::structs::emergency_toggle_struct::EmergencyToggle;


pub fn emergency_toggle_handler(ctx: Context<EmergencyToggle>, paused: bool) -> Result<()> {
    update_program_status(&mut ctx.accounts.program_state, paused)?;
    emit_status_change(&ctx, paused)?;
    log_status_change(&ctx, paused);

    Ok(())
}

fn update_program_status(program_state: &mut Account<ProgramState>, paused: bool) -> Result<()> {
    program_state.paused = paused;
    Ok(())
}

fn emit_status_change(ctx: &Context<EmergencyToggle>, paused: bool) -> Result<()> {
    emit!(ProgramStatusChanged {
        paused,
        authority: ctx.accounts.authority.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

fn log_status_change(ctx: &Context<EmergencyToggle>, paused: bool) {
    msg!(
        "Program {} by authority {}",
        if paused { "paused" } else { "resumed" },
        ctx.accounts.authority.key()
    );
}
