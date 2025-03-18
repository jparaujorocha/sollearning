use anchor_lang::prelude::*;
use crate::states::signers::Multisig;
use crate::states::proposal::{Proposal, ProposalStatus, ProposalInstruction, ProposalCreated};
use crate::error::SolLearningError;
use crate::constants::*;
use crate::instructions::proposal::create_proposal_struct::CreateProposal;

pub fn create_proposal_handler(
    ctx: Context<CreateProposal>,
    instruction: ProposalInstruction,
    description: String,
) -> Result<()> {
    validate_description(&description)?;

    let multisig: &mut Account<'_, Multisig> = &mut ctx.accounts.multisig;
    let proposal_index: u64 = increment_proposal_count(multisig)?;
    let current_time: i64 = Clock::get()?.unix_timestamp;
    let bump = get_proposal_bump(ctx.program_id, multisig.key(), proposal_index)?;

    initialize_proposal(
        &mut ctx.accounts.proposal,
        multisig.key(),
        proposal_index,
        instruction.clone(),
        ctx.accounts.proposer.key(),
        &description,
        bump,
        current_time,
        &multisig.signers,
    )?;

    emit_proposal_created(
        multisig.key(),
        ctx.accounts.proposal.key(),
        ctx.accounts.proposer.key(),
        instruction,
        &description,
        current_time,
    )?;

    msg!(
        "Created proposal #{} in multisig {}",
        proposal_index,
        multisig.key()
    );

    Ok(())
}

// Ensures the proposal description is within the allowed length
fn validate_description(description: &str) -> Result<()> {
    require!(description.len() <= MAX_DESCRIPTION_LENGTH, SolLearningError::DescriptionTooLong);
    Ok(())
}

// Increments the proposal count and checks for overflow
fn increment_proposal_count(multisig: &mut Account<Multisig>) -> Result<u64> {
    let proposal_index = multisig.proposal_count;
    multisig.proposal_count = multisig
        .proposal_count
        .checked_add(1)
        .ok_or(SolLearningError::Overflow)?;
    Ok(proposal_index)
}

// Retrieves the bump for the proposal PDA
fn get_proposal_bump(program_id: &Pubkey, multisig_key: Pubkey, proposal_index: u64) -> Result<u8> {
    let (_, bump) = Pubkey::find_program_address(
        &[
            PROPOSAL_SEED,
            multisig_key.as_ref(),
            &proposal_index.to_le_bytes(),
        ],
        program_id,
    );
    Ok(bump)
}

// Initializes the proposal account
fn initialize_proposal(
    proposal: &mut Account<Proposal>,
    multisig_key: Pubkey,
    index: u64,
    instruction: ProposalInstruction,
    proposer_key: Pubkey,
    description: &str,
    bump: u8,
    timestamp: i64,
    signers: &[Pubkey],
) -> Result<()> {
    proposal.multisig = multisig_key;
    proposal.index = index;
    proposal.instruction = instruction;
    proposal.status = ProposalStatus::Active;
    proposal.created_at = timestamp;
    proposal.closed_at = None;
    proposal.description = description.to_string();
    proposal.bump = bump;
    proposal.signers = signers.iter().map(|&signer| signer == proposer_key).collect();
    Ok(())
}

// Emits the event for proposal creation
fn emit_proposal_created(
    multisig_key: Pubkey,
    proposal_key: Pubkey,
    proposer_key: Pubkey,
    instruction: ProposalInstruction,
    description: &str,
    timestamp: i64,
) -> Result<()> {
    emit!(ProposalCreated {
        multisig: multisig_key,
        proposal: proposal_key,
        proposer: proposer_key,
        instruction,
        description: description.to_string(),
        timestamp,
    });
    Ok(())
}
