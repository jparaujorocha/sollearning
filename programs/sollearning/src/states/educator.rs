use anchor_lang::prelude::*;

#[account]
pub struct EducatorAccount {
    pub educator_address: Pubkey,
    pub authority: Pubkey,
    pub mint_limit: u64,
    pub total_minted: u64,
    pub course_count: u16,
    pub is_active: bool,
    pub created_at: i64,
    pub last_updated_at: i64, 
    pub bump: u8,
}

#[event]
pub struct EducatorRegistered {
    pub educator: Pubkey,
    pub authority: Pubkey,
    pub mint_limit: u64,
    pub timestamp: i64,
}
