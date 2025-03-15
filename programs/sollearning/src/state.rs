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
    pub bump: u8,
}

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

#[account]
pub struct StudentInfo {
    pub student_address: Pubkey,
    pub total_earned: u64,
    pub courses_completed: u32,
    pub last_activity: i64,
    pub bump: u8,
}

#[account]
pub struct Course {
    pub course_id: String,
    pub course_name: String,
    pub educator: Pubkey,
    pub reward_amount: u64,
    pub completion_count: u32,
    pub is_active: bool,
    pub metadata_hash: [u8; 32], // SHA-256 hash of off-chain metadata
    pub created_at: i64,
    pub last_updated_at: i64,
    pub bump: u8,
}

#[account]
pub struct CourseCompletion {
    pub student: Pubkey,
    pub course_id: String,
    pub verified_by: Pubkey,
    pub completion_time: i64,
    pub tokens_awarded: u64,
    pub bump: u8,
}

// Event definitions for program auditing
#[event]
pub struct EducatorRegistered {
    pub educator: Pubkey,
    pub authority: Pubkey,
    pub mint_limit: u64,
    pub timestamp: i64,
}

#[event]
pub struct StudentRegistered {
    pub student: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct CourseCreated {
    pub course_id: String,
    pub course_name: String,
    pub educator: Pubkey,
    pub reward_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct CourseCompleted {
    pub student: Pubkey,
    pub course_id: String,
    pub educator: Pubkey,
    pub tokens_awarded: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensMinted {
    pub recipient: Pubkey,
    pub amount: u64,
    pub minted_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokensBurned {
    pub burner: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct ProgramStatusChanged {
    pub paused: bool,
    pub authority: Pubkey,
    pub timestamp: i64,
}