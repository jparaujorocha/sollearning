use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use crate::state::ProgramState;
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferInstruction<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        constraint = token::accessor::mint(&from)? == token_mint.key() @ SolLearningError::InvalidMint,
        constraint = token::accessor::authority(&from)? == sender.key() @ SolLearningError::Unauthorized,
    )]
    pub from: AccountInfo<'info>,

    #[account(
        mut,
        constraint = token::accessor::mint(&to)? == token_mint.key() @ SolLearningError::InvalidMint,
    )]
    pub to: AccountInfo<'info>,

    #[account(address = program_state.token_mint)]
    pub token_mint: AccountInfo<'info>,

    #[account(
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}

pub fn transfer_handler(ctx: Context<TransferInstruction>, amount: u64) -> Result<()> {
    require!(amount > 0, SolLearningError::InvalidAmount);
    
    let from_balance = token::accessor::amount(&ctx.accounts.from)?;
    require!(from_balance >= amount, SolLearningError::InsufficientBalance);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
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
        ctx.accounts.sender.key(),
        ctx.accounts.to.key()
    );

    Ok(())
}
