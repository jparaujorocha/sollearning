use anchor_lang::prelude::*;
use crate::states::emergency::{EmergencyMultisig, EmergencyMultisigCreated};
use crate::error::SolLearningError;
use crate::constants::*;
use std::collections::HashSet;
use crate::instructions::structs::create_emergency_multisig_struct::CreateEmergencyMultisig;
use crate::utils::pause::{check_program_running, check_authority};

pub fn create_emergency_multisig_handler(
    ctx: Context<CreateEmergencyMultisig>,
    signers: Vec<Pubkey>,
    threshold: u8,
) -> Result<()> {
    check_program_running(&ctx.accounts.program_state)?;
    check_authority(&ctx.accounts.program_state, &ctx.accounts.authority)?;
    
    validate_multisig(&signers, threshold)?;
    validate_unique_signers(&signers)?;
    validate_authority_inclusion(&signers, ctx.accounts.authority.key())?;

    let bump = ctx.bumps.emergency_multisig;
    let current_time = Clock::get()?.unix_timestamp;

    initialize_multisig(
        &mut ctx.accounts.emergency_multisig,
        &signers, 
        threshold,
        ctx.accounts.authority.key(),
        bump,
    )?;

    emit!(EmergencyMultisigCreated {
        multisig: ctx.accounts.emergency_multisig.key(),
        signers: signers.clone(),
        threshold,
        timestamp: current_time,
    });

    msg!(
        "Emergency multisig created with {} signers and threshold of {}",
        signers.len(),
        threshold
    );

    Ok(())
}

fn validate_multisig(signers: &[Pubkey], threshold: u8) -> Result<()> {
    require!(!signers.is_empty(), SolLearningError::InvalidMultisigConfig);
    require!(signers.len() <= MAX_SIGNERS, SolLearningError::MaxSignersReached);
    require!(
        threshold > 0 && (threshold as usize) <= signers.len(),
        SolLearningError::InvalidThreshold
    );
    Ok(())
}

fn validate_unique_signers(signers: &[Pubkey]) -> Result<()> {
    let mut unique_signers = HashSet::new();
    for signer in signers {
        require!(unique_signers.insert(*signer), SolLearningError::SignerAlreadyExists);
    }
    Ok(())
}

fn validate_authority_inclusion(signers: &[Pubkey], authority: Pubkey) -> Result<()> {
    require!(signers.contains(&authority), SolLearningError::UnauthorizedAuthority);
    Ok(())
}

fn initialize_multisig(
    multisig: &mut Account<EmergencyMultisig>,
    signers: &[Pubkey],
    threshold: u8,
    authority: Pubkey,
    bump: u8,
) -> Result<()> {
    multisig.signers = signers.to_vec();
    multisig.threshold = threshold;
    multisig.proposal_count = 0;
    multisig.authority = authority;
    multisig.bump = bump;
    Ok(())
}