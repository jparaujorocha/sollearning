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
    
    #[msg("Invalid multisig configuration")]
    InvalidMultisigConfig,
    
    #[msg("Threshold must be greater than 0 and less than or equal to the number of signers")]
    InvalidThreshold,
    
    #[msg("Signer already exists in multisig")]
    SignerAlreadyExists,
    
    #[msg("Signer does not exist in multisig")]
    SignerDoesNotExist,
    
    #[msg("Maximum number of signers reached")]
    MaxSignersReached,
    
    #[msg("Not enough signers to execute proposal")]
    NotEnoughSigners,
    
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    
    #[msg("Proposal already cancelled")]
    ProposalAlreadyCancelled,
    
    #[msg("Proposal has expired")]
    ProposalExpired,
    
    #[msg("Invalid proposal status")]
    InvalidProposalStatus,
    
    #[msg("Signer has already approved this proposal")]
    AlreadyApproved,
    
    #[msg("Description too long")]
    DescriptionTooLong,
    
    #[msg("Removing this signer would make threshold impossible to reach")]
    CannotRemoveSigner,
    
    #[msg("Cannot remove last signer")]
    CannotRemoveLastSigner,
    
    #[msg("Cannot create course history record")]
    CannotCreateCourseHistory,
}