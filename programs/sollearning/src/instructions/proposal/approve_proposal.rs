use anchor_lang::prelude::*;
use crate::states::signers::Multisig;
use crate::states::proposal::{Proposal, ProposalStatus, ProposalApproved};
use crate::error::SolLearningError;
use crate::constants::*;
use crate::instructions::proposal::approve_proposal_struct::ApproveProposal;

pub fn approve_proposal_handler(ctx: Context<ApproveProposal>) -> Result<()> {
    let proposal: &mut Account<'_, Proposal> = &mut ctx.accounts.proposal;
    let multisig: &Account<'_, Multisig> = &ctx.accounts.multisig;
    let signer_key: Pubkey = ctx.accounts.signer.key();

    let current_time: i64 = Clock::get()?.unix_timestamp;

    check_proposal_expiration(proposal, current_time)?;
    let signer_index: usize = find_signer_index(multisig, signer_key)?;

    validate_approval(proposal, signer_index)?;

    proposal.signers[signer_index] = true;

    emit_proposal_approved(multisig.key(), proposal.key(), signer_key, current_time)?;

    msg!(
        "Proposal #{} approved by signer {}",
        proposal.index,
        signer_key
    );

    Ok(())
}

fn check_proposal_expiration(proposal: &mut Account<Proposal>, current_time: i64) -> Result<()> {
    if current_time - proposal.created_at > PROPOSAL_EXPIRATION_TIME {
        proposal.status = ProposalStatus::Expired;
        proposal.closed_at = Some(current_time);
        return Err(SolLearningError::ProposalExpired.into());
    }
    Ok(())
}

fn find_signer_index(multisig: &Account<Multisig>, signer_key: Pubkey) -> Result<usize> {
    multisig
        .signers
        .iter()
        .position(|&key| key == signer_key)
        .ok_or(SolLearningError::Unauthorized.into())
}

fn validate_approval(proposal: &Account<Proposal>, signer_index: usize) -> Result<()> {
    require!(
        !proposal.signers[signer_index],
        SolLearningError::AlreadyApproved
    );
    Ok(())
}

fn emit_proposal_approved(multisig_key: Pubkey, proposal_key: Pubkey, signer_key: Pubkey, timestamp: i64) -> Result<()> {
    emit!(ProposalApproved {
        multisig: multisig_key,
        proposal: proposal_key,
        signer: signer_key,
        timestamp,
    });
    Ok(())
}
