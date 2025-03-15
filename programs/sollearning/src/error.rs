use anchor_lang::prelude::*;

#[error_code]
pub enum SolLearningError {
    #[msg("Acesso não autorizado")]
    Unauthorized,
    
    #[msg("Quantidade de tokens inválida")]
    InvalidAmount,
    
    #[msg("Saldo insuficiente de tokens")]
    InsufficientBalance,
    
    #[msg("Token mint inválido")]
    InvalidMint,
    
    #[msg("Overflow na operação aritmética")]
    Overflow,
    
    #[msg("A conta de tokens já foi inicializada")]
    AlreadyInitialized,
    
    #[msg("Autoridade inválida")]
    InvalidAuthority,
    
    #[msg("Conta de educador inválida")]
    InvalidEducator,
    
    #[msg("Conta de estudante inválida")]
    InvalidStudent,
    
    #[msg("O curso já foi concluído pelo estudante")]
    CourseAlreadyCompleted,
    
    #[msg("ID do curso inválido")]
    InvalidCourseId,
    
    #[msg("Educador inativo")]
    InactiveEducator,
    
    #[msg("A conta já está registrada")]
    AlreadyRegistered,
    
    #[msg("O programa está pausado")]
    ProgramPaused,
}
