use anchor_lang::prelude::*;
use crate::states::signers::Multisig;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(signers: Vec<Pubkey>, threshold: u8)]
pub struct CreateMultisig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = calculate_multisig_space(&signers), 
        seeds = [MULTISIG_SEED],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,

    pub system_program: Program<'info, System>,
}

pub fn calculate_multisig_space(signers: &[Pubkey]) -> usize {
    8 + 4 + (signers.len() * 32) + 1 + 8 + 32 + 1
}
