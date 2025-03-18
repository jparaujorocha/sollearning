use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::emergency::EmergencyMultisig;
use crate::constants::*;
use crate::error::SolLearningError;

#[derive(Accounts)]
#[instruction(signers: Vec<Pubkey>, threshold: u8)]
pub struct CreateEmergencyMultisig<'info> {
    #[account(
        mut,
        constraint = authority.key() == program_state.authority @ SolLearningError::UnauthorizedAuthority
    )]
    pub authority: Signer<'info>,

    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = authority,
        space = 8 + 4 + (signers.len() * 32) + 1 + 8 + 32 + 1, 
        seeds = [EMERGENCY_MULTISIG_SEED],
        bump,
    )]
    pub emergency_multisig: Account<'info, EmergencyMultisig>,

    pub system_program: Program<'info, System>,
}