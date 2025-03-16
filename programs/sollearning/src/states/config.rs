use anchor_lang::prelude::*;

#[account]
pub struct ProgramConfig {
    pub max_educators: u16,
    pub max_courses_per_educator: u16,
    pub max_mint_amount: u64,
    pub mint_cooldown_period: i64,
    pub authority: Pubkey,
    pub last_updated_at: i64,
    pub bump: u8,
}

#[event]
pub struct ConfigUpdated {
    pub authority: Pubkey,
    pub timestamp: i64,
}