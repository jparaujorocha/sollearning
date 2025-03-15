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
    /// CHECK: We verify through constraints that this is a valid token account for the sender
    pub from: AccountInfo<'info>,

    #[account(
        mut,
        constraint = token::accessor::mint(&to)? == token_mint.key() @ SolLearningError::InvalidMint,
    )]
    /// CHECK: We verify that this is a valid token account with the correct mint
    pub to: AccountInfo<'info>,

    /// CHECK: We verify that this is a token account with the correct mint
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
    // Validate the transfer amount
    require!(amount > 0, SolLearningError::InvalidAmount);
    
    // Get current balances for improved logging and front-running protection
    let from_balance = token::accessor::amount(&ctx.accounts.from)?;
    let to_balance = token::accessor::amount(&ctx.accounts.to)?;
    
    // Verify sender has sufficient balance with buffer to prevent front-running
    let buffer_amount = amount / 10; // 10% buffer
    require!(
        from_balance >= amount.checked_add(buffer_amount).ok_or(SolLearningError::Overflow)?, 
        SolLearningError::TransferFrontRunning
    );

    // Execute the transfer
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

    // Calculate new balances for improved logging
    let new_from_balance = from_balance - amount;
    let new_to_balance = to_balance + amount;

    msg!(
        "Transferred {} tokens from {} to {} - Sender previous balance: {}, new balance: {} | Recipient previous balance: {}, new balance: {}",
        amount,
        ctx.accounts.sender.key(),
        ctx.accounts.to.key(),
        from_balance,
        new_from_balance,
        to_balance,
        new_to_balance
    );

    Ok(())
}