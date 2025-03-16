use anchor_lang::prelude::*;
use crate::state::{Multisig, Proposal, ProposalInstruction, ProposalStatus, ProposalCreated};
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(instruction: ProposalInstruction, description: String)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [MULTISIG_SEED],
        bump = multisig.bump,
        constraint = multisig.signers.contains(&proposer.key()) @ SolLearningError::Unauthorized,
    )]
    pub multisig: Account<'info, Multisig>,
    
    #[account(
        init,
        payer = proposer,
        space = 8 + 32 + 8 + 100 + 4 + (multisig.signers.len()) + 1 + 8 + 8 + 4 + description.len() + 1,
        seeds = [PROPOSAL_SEED, multisig.key().as_ref(), &multisig.proposal_count.to_le_bytes()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_proposal_handler(
    ctx: Context<CreateProposal>,
    instruction: ProposalInstruction,
    description: String,
) -> Result<()> {
    // Validate description length
    require!(description.len() <= MAX_DESCRIPTION_LENGTH, SolLearningError::DescriptionTooLong);
    
    // Get the current proposal index and increment it
    let multisig = &mut ctx.accounts.multisig;
    let proposal_index = multisig.proposal_count;
    multisig.proposal_count = multisig.proposal_count.checked_add(1).ok_or(SolLearningError::Overflow)?;
    
    // Get current timestamp
    let current_time = Clock::get()?.unix_timestamp;
    
    // Get bump
    let (_, bump) = Pubkey::find_program_address(
        &[
            PROPOSAL_SEED,
            multisig.key().as_ref(),
            &proposal_index.to_le_bytes(),
        ],
        ctx.program_id
    );
    
    // Initialize proposal account
    let proposal = &mut ctx.accounts.proposal;
    proposal.multisig = multisig.key();
    proposal.index = proposal_index;
    proposal.instruction = instruction.clone();
    
    // Initialize signers approval status (all false)
    let mut signers_approval = Vec::with_capacity(multisig.signers.len());
    for i in 0..multisig.signers.len() {
        // Auto-approve the proposal for the proposer
        let is_approved = multisig.signers[i] == ctx.accounts.proposer.key();
        signers_approval.push(is_approved);
    }
    
    proposal.signers = signers_approval;
    proposal.status = ProposalStatus::Active;
    proposal.created_at = current_time;
    proposal.closed_at = None;
    proposal.description = description.clone();
    proposal.bump = bump;
    
    // Emit proposal creation event
    emit!(ProposalCreated {
        multisig: multisig.key(),
        proposal: proposal.key(),
        proposer: ctx.accounts.proposer.key(),
        instruction: instruction.clone(),
        description: description.clone(),
        timestamp: current_time,
    });
    
    msg!(
        "Created proposal #{} in multisig {}",
        proposal_index,
        multisig.key()
    );
    
    Ok(())
}