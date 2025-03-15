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
    
    #[msg("Course already completed by student")]
    CourseAlreadyCompleted,
    
    #[msg("Invalid course ID")]
    InvalidCourseId,
    
    #[msg("Inactive educator")]
    InactiveEducator,
    
    #[msg("Account already registered")]
    AlreadyRegistered,
    
    #[msg("Program is paused")]
    ProgramPaused,

    #[msg("Course name too long")]
    CourseNameTooLong,

    #[msg("Course ID too long")]
    CourseIdTooLong,

    #[msg("Maximum educators limit reached")]
    MaxEducatorsLimitReached,

    #[msg("Maximum courses per educator limit reached")]
    MaxCoursesPerEducatorReached,

    #[msg("Course does not exist")]
    CourseDoesNotExist,

    #[msg("Course is inactive")]
    CourseInactive,

    #[msg("Course does not belong to educator")]
    CourseNotOwnedByEducator,

    #[msg("Invalid course reward amount")]
    InvalidCourseReward,
    
    #[msg("Only authority can burn tokens")]
    BurnNotAuthorized,
    
    #[msg("Minting tokens too frequently - please wait for cooldown period")]
    MintingTooFrequent,
    
    #[msg("Insufficient balance for transfer - transaction may be front-running")]
    TransferFrontRunning,
}