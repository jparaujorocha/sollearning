use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::signers::Multisig;
use crate::states::proposal::Proposal;

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,

    #[account(mut)]
    pub multisig: Account<'info, Multisig>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
}
