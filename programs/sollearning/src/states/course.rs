use anchor_lang::prelude::*;

#[account]
pub struct Course {
    pub course_id: String,
    pub course_name: String,
    pub educator: Pubkey,
    pub reward_amount: u64,
    pub completion_count: u32,
    pub is_active: bool,
    pub metadata_hash: [u8; 32],
    pub created_at: i64,
    pub last_updated_at: i64,
    pub version: u32, 
    pub bump: u8,
}

#[account]
pub struct CourseHistory {
    pub course_id: String,
    pub educator: Pubkey,
    pub version: u32,
    pub previous_name: String,
    pub previous_reward: u64,
    pub previous_active: bool,
    pub previous_metadata_hash: [u8; 32],
    pub updated_by: Pubkey,
    pub updated_at: i64,
    pub change_description: String,
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

#[event]
pub struct CourseCreated {
    pub course_id: String,
    pub course_name: String,
    pub educator: Pubkey,
    pub reward_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct CourseUpdated {
    pub course_id: String,
    pub educator: Pubkey,
    pub version: u32,
    pub previous_name: String,
    pub new_name: Option<String>,
    pub previous_reward: u64,
    pub new_reward: Option<u64>,
    pub previous_active: bool,
    pub new_active: Option<bool>,
    pub updated_by: Pubkey,
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