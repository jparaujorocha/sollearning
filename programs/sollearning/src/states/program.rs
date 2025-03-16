use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ProgramState {
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub total_minted: u64,
    pub total_burned: u64,
    pub educator_count: u16,
    pub paused: bool,
    pub pause_flags: u32,
    pub bump: u8,
}

#[event]
pub struct ProgramStatusChanged {
    pub paused: bool,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProgramGranularPauseChanged {
    pub pause_flags: u32,
    pub authority: Pubkey,
    pub timestamp: i64,
}