use anchor_lang::prelude::*;
use crate::states::program::{ProgramState, ProgramStatusChanged, ProgramGranularPauseChanged};
use crate::instructions::structs::emergency_toggle_struct::{EmergencyToggle, EmergencyToggleGranular, EmergencyMultisigToggle};
use crate::utils::pause::check_authority;
use crate::constants::*;

pub fn emergency_toggle_handler(ctx: Context<EmergencyToggle>, paused: bool) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Explicit authority check
    check_authority(&ctx.accounts.program_state, &ctx.accounts.authority)?;
    
    update_program_status(&mut ctx.accounts.program_state, paused)?;
    
    emit_status_change(&ctx, paused, current_time)?;
    
    msg!(
        "Program {} by authority {}",
        if paused { "paused" } else { "resumed" },
        ctx.accounts.authority.key()
    );

    Ok(())
}

pub fn emergency_toggle_granular_handler(
    ctx: Context<EmergencyToggleGranular>,
    function_flags: u32,
    set_flags: bool
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Explicit authority check
    check_authority(&ctx.accounts.program_state, &ctx.accounts.authority)?;
    
    let new_flags = update_pause_flags(
        &mut ctx.accounts.program_state,
        function_flags,
        set_flags
    )?;
    
    emit_granular_status_change(&ctx, new_flags, current_time)?;
    
    msg!(
        "Function flags {} by authority {}: new flags: 0x{:08X}",
        if set_flags { "set" } else { "cleared" },
        ctx.accounts.authority.key(),
        new_flags
    );

    Ok(())
}

pub fn emergency_multisig_toggle_handler(
    ctx: Context<EmergencyMultisigToggle>,
    paused: bool
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    update_program_status(&mut ctx.accounts.program_state, paused)?;
    
    emit!(ProgramStatusChanged {
        paused,
        authority: ctx.accounts.multisig_authority.key(),
        timestamp: current_time,
    });
    
    msg!(
        "Program {} by emergency multisig authority {}",
        if paused { "paused" } else { "resumed" },
        ctx.accounts.multisig_authority.key()
    );

    Ok(())
}

fn update_program_status(program_state: &mut Account<ProgramState>, paused: bool) -> Result<()> {
    program_state.paused = paused;
    
    // If pausing the program, also set all function flags
    if paused {
        program_state.pause_flags = PAUSE_FLAG_ALL;
    } else {
        program_state.pause_flags = 0;
    }
    
    Ok(())
}

fn update_pause_flags(
    program_state: &mut Account<ProgramState>,
    function_flags: u32,
    set_flags: bool
) -> Result<u32> {
    if set_flags {
        program_state.pause_flags |= function_flags;
    } else {
        program_state.pause_flags &= !function_flags;
    }
    
    Ok(program_state.pause_flags)
}

fn emit_status_change(
    ctx: &Context<EmergencyToggle>,
    paused: bool,
    timestamp: i64
) -> Result<()> {
    emit!(ProgramStatusChanged {
        paused,
        authority: ctx.accounts.authority.key(),
        timestamp,
    });
    Ok(())
}

fn emit_granular_status_change(
    ctx: &Context<EmergencyToggleGranular>,
    pause_flags: u32,
    timestamp: i64
) -> Result<()> {
    emit!(ProgramGranularPauseChanged {
        pause_flags,
        authority: ctx.accounts.authority.key(),
        timestamp,
    });
    Ok(())
}