use anchor_lang::prelude::*;
use crate::states::config::ProgramConfig;
use crate::error::SolLearningError;

pub fn get_max_educators(config: &Account<ProgramConfig>) -> u16 {
    config.max_educators
}

pub fn get_max_courses_per_educator(config: &Account<ProgramConfig>) -> u16 {
    config.max_courses_per_educator
}

pub fn get_max_mint_amount(config: &Account<ProgramConfig>) -> u64 {
    config.max_mint_amount
}

pub fn get_mint_cooldown_period(config: &Account<ProgramConfig>) -> i64 {
    config.mint_cooldown_period
}

pub fn check_authority(config: &Account<ProgramConfig>, authority: &Signer) -> Result<()> {
    require!(
        config.authority == authority.key(),
        SolLearningError::UnauthorizedAuthority
    );
    Ok(())
}