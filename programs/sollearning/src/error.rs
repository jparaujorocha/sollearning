use anchor_lang::prelude::*;

#[error_code]
pub enum SolLearningError {
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid token amount")]
    InvalidAmount,
    
    #[msg("Insufficient token balance")]
    InsufficientBalance,
    
    #[msg("Invalid token mint")]
    InvalidMint,
    
    #[msg("Arithmetic operation overflow")]
    Overflow,
    
    #[msg("Token account already initialized")]
    AlreadyInitialized,
    
    #[msg("Invalid authority")]
    InvalidAuthority,
    
    #[msg("Invalid educator account")]
    InvalidEducator,
    
    #[msg("Invalid student account")]
    InvalidStudent,
}