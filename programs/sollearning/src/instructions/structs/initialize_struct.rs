use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::program::ProgramState;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<ProgramState>(),
        seeds = [PROGRAM_STATE_SEED],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = authority,
        space = 8 + 82,
    )]
    pub token_mint: Account<'info, Mint>,

    /// CHECK: This account should be created by the associated token program
    #[account(mut)]
    pub authority_token_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
