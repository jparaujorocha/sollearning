use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::error::SolLearningError;

#[derive(Accounts)]
pub struct EmergencyToggle<'info> {
    #[account(
        mut,
        constraint = authority.key() == program_state.authority @ SolLearningError::UnauthorizedAuthority
    )]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
}

#[derive(Accounts)]
pub struct EmergencyToggleGranular<'info> {
    #[account(
        mut,
        constraint = authority.key() == program_state.authority @ SolLearningError::UnauthorizedAuthority
    )]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
}

#[derive(Accounts)]
pub struct EmergencyMultisigToggle<'info> {
    #[account(mut)]
    pub multisig_authority: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        constraint = emergency_multisig.signers.contains(&multisig_authority.key()) @ SolLearningError::Unauthorized
    )]
    pub emergency_multisig: Account<'info, crate::states::emergency::EmergencyMultisig>,
}