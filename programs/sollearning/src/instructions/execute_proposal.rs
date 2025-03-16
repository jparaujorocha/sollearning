use anchor_lang::prelude::*;
use crate::state::{ProgramState, Multisig, Proposal, ProposalStatus, ProposalInstruction, ProposalExecuted, EducatorAccount};
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,

    #[account(
        mut,
        seeds = [MULTISIG_SEED],
        bump = multisig.bump,
        constraint = multisig.signers.contains(&executor.key()) @ SolLearningError::Unauthorized,
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

    #[account(
        mut,
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
    )]
    pub program_state: Account<'info, ProgramState>,
}

pub fn execute_proposal_handler(ctx: Context<ExecuteProposal>) -> Result<()> {
    require!(ctx.accounts.proposal.signers.iter().filter(|&&s| s).count() >= ctx.accounts.multisig.threshold as usize,
        SolLearningError::NotEnoughSigners);

    ctx.accounts.proposal.status = ProposalStatus::Executed;

    emit!(ProposalExecuted {
        multisig: ctx.accounts.multisig.key(),
        proposal: ctx.accounts.proposal.key(),
        executor: ctx.accounts.executor.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Proposal #{} executed by {}", ctx.accounts.proposal.index, ctx.accounts.executor.key());

    Ok(())
}
