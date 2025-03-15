use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use crate::state::ProgramState;
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
    /// CHECK: Verificamos através de constraints que esta é uma conta de token válida 
    pub token_account: AccountInfo<'info>,

    /// CHECK: Verificamos através de constraints que esta é uma conta de token válida 
    #[account(
        mut,
        address = program_state.token_mint,
    )]

    /// CHECK: Verificamos através do constraint address que esta é a conta de mint correta
    pub token_mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub program_state: Account<'info, ProgramState>,

    pub token_program: Program<'info, Token>,
}

pub fn burn_handler(ctx: Context<BurnInstruction>, amount: u64) -> Result<()> {
    require!(amount > 0, SolLearningError::InvalidAmount);

    let token_balance = token::accessor::amount(&ctx.accounts.token_account)?;
    require!(
        token_balance >= amount,
        SolLearningError::InsufficientBalance
    );

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

    let program_state = &mut ctx.accounts.program_state;
    program_state.total_burned = program_state
        .total_burned
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;

    msg!(
        "Burned {} tokens from {}",
        amount,
        ctx.accounts.owner.key()
    );

    Ok(())
}
