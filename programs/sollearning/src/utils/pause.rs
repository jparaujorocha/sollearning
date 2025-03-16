use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::error::SolLearningError;

pub fn check_program_running(program_state: &Account<ProgramState>) -> Result<()> {
    require!(!program_state.paused, SolLearningError::ProgramPaused);
    Ok(())
}

pub fn check_function_running(program_state: &Account<ProgramState>, function_flag: u32) -> Result<()> {
    require!(
        (program_state.pause_flags & function_flag) == 0,
        SolLearningError::FunctionPaused
    );
    Ok(())
}

pub fn check_authority(program_state: &Account<ProgramState>, authority: &Signer) -> Result<()> {
    require!(
        program_state.authority == authority.key(),
        SolLearningError::UnauthorizedAuthority
    );
    Ok(())
}