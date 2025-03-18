use anchor_lang::prelude::*;
use crate::states::proposal::{Proposal, ProposalStatus, ProposalExecuted};
use crate::error::SolLearningError;
use crate::instructions::proposal::execute_proposal_struct::ExecuteProposal;

pub fn execute_proposal_handler(ctx: Context<ExecuteProposal>) -> Result<()> {
    validate_execution(&ctx)?;

    update_proposal_status(&mut ctx.accounts.proposal);

    emit_proposal_executed(&ctx)?;

    log_execution(&ctx);

    Ok(())
}

fn validate_execution(ctx: &Context<ExecuteProposal>) -> Result<()> {
    let approvals = ctx.accounts.proposal.signers.iter().filter(|&&s| s).count();
    require!(
        approvals >= ctx.accounts.multisig.threshold as usize,
        SolLearningError::NotEnoughSigners
    );
    Ok(())
}

fn update_proposal_status(proposal: &mut Account<Proposal>) {
    proposal.status = ProposalStatus::Executed;
}

fn emit_proposal_executed(ctx: &Context<ExecuteProposal>) -> Result<()> {
    emit!(ProposalExecuted {
        multisig: ctx.accounts.multisig.key(),
        proposal: ctx.accounts.proposal.key(),
        executor: ctx.accounts.executor.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

fn log_execution(ctx: &Context<ExecuteProposal>) {
    msg!(
        "Proposal #{} executed by {}",
        ctx.accounts.proposal.index,
        ctx.accounts.executor.key()
    );
}