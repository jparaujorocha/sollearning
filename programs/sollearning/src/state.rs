use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ProgramState {
    /// The token mint address
    pub token_mint: Pubkey,
    
    /// The authority with admin rights
    pub authority: Pubkey,
    
    /// Total tokens minted so far
    pub total_minted: u64,
    
    /// Total tokens burned
    pub total_burned: u64,
    
    /// Bump for the program state account
    pub bump: u8,
}

#[account]
pub struct EducatorAccount {
    /// The educator's address
    pub educator_address: Pubkey,
    
    /// The authority that created this educator
    pub authority: Pubkey,
    
    /// Maximum amount this educator can mint per instruction
    pub mint_limit: u64,
    
    /// Total amount minted by this educator
    pub total_minted: u64,
    
    /// Whether this educator is active
    pub is_active: bool,
    
    /// Account bump
    pub bump: u8,
}

#[account]
pub struct StudentInfo {
    /// The student's address
    pub student_address: Pubkey,
    
    /// Total tokens earned by this student
    pub total_earned: u64,
    
    /// Total courses completed
    pub courses_completed: u32,
    
    /// Last activity timestamp
    pub last_activity: i64,
    
    /// Account bump
    pub bump: u8,
}