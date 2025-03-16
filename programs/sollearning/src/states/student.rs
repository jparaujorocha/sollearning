use anchor_lang::prelude::*;

#[account]
pub struct StudentInfo {
    pub student_address: Pubkey,
    pub total_earned: u64,
    pub courses_completed: u32,
    pub last_activity: i64,
    pub bump: u8,
}

#[event]
pub struct StudentRegistered {
    pub student: Pubkey,
    pub timestamp: i64,
}
