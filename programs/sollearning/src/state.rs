use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ProgramState {
    pub token_mint: Pubkey,
    pub authority: Pubkey,
    pub total_minted: u64,
    pub total_burned: u64,
    pub paused: bool,
    pub bump: u8,
}

#[account]
pub struct EducatorAccount {
    pub educator_address: Pubkey,
    pub authority: Pubkey,
    pub mint_limit: u64,
    pub total_minted: u64,
    pub is_active: bool,
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
pub struct CourseCompletion {
    pub student: Pubkey,
    pub course_id: String,
    pub verified_by: Pubkey,
    pub completion_time: i64,
    pub tokens_awarded: u64,
    pub bump: u8,
}
