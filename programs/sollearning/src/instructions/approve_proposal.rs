use anchor_lang::prelude::*;
use crate::state::{Multisig, Proposal, ProposalStatus, ProposalApproved};
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
pub struct ApproveProposal<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(
        seeds = [MULTISIG_SEED],
        bump = multisig.bump,
        constraint = multisig.signers.contains(&signer.key()) @ SolLearningError::Unauthorized,
    )]
    pub multisig: Account<'info, Multisig>,
    
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, multisig.key().as_ref(), &proposal.index.to_le_bytes()],
        bump = proposal.bump,
        constraint = proposal.multisig == multisig.key() @ SolLearningError::InvalidMultisigConfig,
        constraint = proposal.status == ProposalStatus::Active @ SolLearningError::InvalidProposalStatus,
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn approve_proposal_handler(ctx: Context<ApproveProposal>) -> Result<()> {
    // Get the current timestamp
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if the proposal has expired
    if current_time - ctx.accounts.proposal.created_at > PROPOSAL_EXPIRATION_TIME {
        ctx.accounts.proposal.status = ProposalStatus::Expired;
        ctx.accounts.proposal.closed_at = Some(current_time);
        return Err(SolLearningError::ProposalExpired.into());
    }
    
    // Find the signer's index in the multisig signers list
    let mut signer_index = None;
    for (i, key) in ctx.accounts.multisig.signers.iter().enumerate() {
        if *key == ctx.accounts.signer.key() {
            signer_index = Some(i);
            break;
        }
    }
    
    let signer_index = signer_index.ok_or(SolLearningError::Unauthorized)?;
    
    // Check if the signer has already approved
    require!(
        !ctx.accounts.proposal.signers[signer_index],
        SolLearningError::AlreadyApproved
    );
    
    // Approve the proposal
    ctx.accounts.proposal.signers[signer_index] = true;
    
    // Emit approval event
    emit!(ProposalApproved {
        multisig: ctx.accounts.multisig.key(),
        proposal: ctx.accounts.proposal.key(),
        signer: ctx.accounts.signer.key(),
        timestamp: current_time,
    });
    
    msg!(
        "Proposal #{} approved by signer {}",
        ctx.accounts.proposal.index,
        ctx.accounts.signer.key()
    );
    
    Ok(())
}