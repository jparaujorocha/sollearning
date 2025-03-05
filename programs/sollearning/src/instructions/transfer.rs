use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use crate::{
    state::ProgramState,
    error::SolLearningError,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferInstruction<'info> {   // <- Renomeado para não colidir com anchor_spl::Transfer
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        constraint = from.owner == sender.key() @ SolLearningError::Unauthorized,
        constraint = from.mint == program_state.token_mint @ SolLearningError::InvalidMint,
    )]
    pub from: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = to.mint == program_state.token_mint @ SolLearningError::InvalidMint,
    )]
    pub to: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"program-state".as_ref()],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}

pub fn transfer_handler(ctx: Context<TransferInstruction>, amount: u64) -> Result<()> {
    require!(amount > 0, SolLearningError::InvalidAmount);
    require!(ctx.accounts.from.amount >= amount, SolLearningError::InsufficientBalance);

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        amount,
    )?;

    msg!(
        "Transferred {} tokens from {} to {}",
        amount,
        ctx.accounts.from.key(),
        ctx.accounts.to.key()
    );

    Ok(())
}
