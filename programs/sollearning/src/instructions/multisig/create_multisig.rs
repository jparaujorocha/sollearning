use anchor_lang::prelude::*;
use crate::states::signers::{Multisig, MultisigCreated};
use crate::error::SolLearningError;
use crate::constants::*;
use std::collections::HashSet;
use crate::instructions::multisig::create_multisig_struct::CreateMultisig;

pub fn create_multisig_handler(
    ctx: Context<CreateMultisig>,
    signers: Vec<Pubkey>,
    threshold: u8,
) -> Result<()> {
    validate_multisig(&signers, threshold)?;
    validate_unique_signers(&signers)?;
    validate_authority(&signers, ctx.accounts.authority.key())?;

    let bump = Pubkey::find_program_address(&[MULTISIG_SEED], ctx.program_id).1;

    initialize_multisig(
        &mut ctx.accounts.multisig,
        &signers, 
        threshold,
        ctx.accounts.authority.key(),
        bump,
    )?;

    emit_multisig_created(ctx.accounts.multisig.key(), &signers, threshold)?;

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

fn validate_authority(signers: &[Pubkey], authority: Pubkey) -> Result<()> {
    require!(signers.contains(&authority), SolLearningError::Unauthorized);
    Ok(())
}

fn initialize_multisig(
    multisig: &mut Account<Multisig>,
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

fn emit_multisig_created(multisig_key: Pubkey, signers: &[Pubkey], threshold: u8) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    emit!(MultisigCreated {
        multisig: multisig_key,
        signers: signers.to_vec(),
        threshold,
        timestamp,
    });

    msg!(
        "Multisig created with {} signers and threshold of {}",
        signers.len(),
        threshold
    );

    Ok(())
}
