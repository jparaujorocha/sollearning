use anchor_lang::prelude::*;
use crate::states::signers::Multisig;
use crate::states::proposal::{Proposal, ProposalStatus};
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
