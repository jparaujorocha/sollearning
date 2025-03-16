use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use crate::states::program::ProgramState;
use crate::states::course::TokensBurned;
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BurnInstruction<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = token::accessor::mint(&token_account)? == token_mint.key() @ SolLearningError::InvalidMint,
        constraint = token::accessor::authority(&token_account)? == owner.key() @ SolLearningError::Unauthorized,
    )]
    /// CHECK: We verify through constraints that this is a valid token account 
    pub token_account: AccountInfo<'info>,

    /// CHECK: We verify through constraints that this is a valid token account
    #[account(
        mut,
        address = program_state.token_mint,
    )]
    pub token_mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
        // Add constraint to ensure only program authority can burn tokens
        constraint = (owner.key() == program_state.authority) @ SolLearningError::BurnNotAuthorized,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}

pub fn burn_handler(ctx: Context<BurnInstruction>, amount: u64) -> Result<()> {
    // Validate burn amount
    require!(amount > 0, SolLearningError::InvalidAmount);

    // Get current token balance for improved logging
    let token_balance = token::accessor::amount(&ctx.accounts.token_account)?;
    require!(token_balance >= amount, SolLearningError::InsufficientBalance);

    // Execute token burn
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update program state to track burned tokens
    let program_state = &mut ctx.accounts.program_state;
    program_state.total_burned = program_state
        .total_burned
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;

    // Get current timestamp for event
    let current_time = Clock::get()?.unix_timestamp;

    // Calculate new balance for improved logging
    let new_balance = token_balance - amount;

    // Emit token burn event
    emit!(TokensBurned {
        burner: ctx.accounts.owner.key(),
        amount,
        timestamp: current_time,
    });

    msg!(
        "Burned {} tokens from {} by authority - Previous balance: {}, New balance: {}, Total burned: {}",
        amount,
        ctx.accounts.owner.key(),
        token_balance,
        new_balance,
        program_state.total_burned
    );

    Ok(())
}