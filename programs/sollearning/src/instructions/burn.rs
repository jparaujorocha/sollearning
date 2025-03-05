use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Burn};
use crate::{
    state::ProgramState,
    error::SolLearningError,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BurnInstruction<'info> {  // <- Renomeado para não colidir com anchor_spl::Burn
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = token_account.owner == owner.key() @ SolLearningError::Unauthorized,
        constraint = token_account.mint == token_mint.key() @ SolLearningError::InvalidMint,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = program_state.token_mint @ SolLearningError::InvalidMint,
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"program-state".as_ref()],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}

pub fn burn_handler(ctx: Context<BurnInstruction>, amount: u64) -> Result<()> {
    require!(amount > 0, SolLearningError::InvalidAmount);
    require!(ctx.accounts.token_account.amount >= amount, SolLearningError::InsufficientBalance);

    anchor_spl::token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.token_mint.to_account_info(),
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    let program_state = &mut ctx.accounts.program_state;
    program_state.total_burned = program_state
        .total_burned
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;

    msg!(
        "Burned {} tokens from {}",
        amount,
        ctx.accounts.token_account.key()
    );

    Ok(())
}
