use anchor_lang::prelude::*;
use crate::states::signers::Multisig;
use crate::states::proposal::Proposal;
use crate::states::proposal::ProposalInstruction;
use crate::constants::PROPOSAL_SEED;

#[derive(Accounts)]
#[instruction(instruction: ProposalInstruction, description: String)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,

    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(
        init,
        payer = proposer,
        space = calculate_proposal_space(&description, multisig.signers.len()), 
        seeds = [PROPOSAL_SEED, multisig.key().as_ref(), &multisig.proposal_count.to_le_bytes()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

pub fn calculate_proposal_space(description: &str, signers_count: usize) -> usize {
    8 + 32 + 8 + 100 + 4 + signers_count + 1 + 8 + 8 + 4 + description.len() + 1
}
