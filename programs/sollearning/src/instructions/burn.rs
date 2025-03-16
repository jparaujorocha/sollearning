use anchor_lang::prelude::*;
use anchor_spl::token::{self};
use crate::states::program::ProgramState;
use crate::states::course::TokensBurned;
use crate::error::SolLearningError;
use crate::instructions::structs::burn_struct::BurnInstruction;

pub fn burn_handler(ctx: Context<BurnInstruction>, amount: u64) -> Result<()> {
    validate_burn_amount(amount)?;

    let token_balance = token::accessor::amount(&ctx.accounts.token_account)?;
    require!(token_balance >= amount, SolLearningError::InsufficientBalance);

    let new_balance = token_balance - amount;
    
    burn_tokens(&ctx, amount)?;
    update_burned_tokens(&mut ctx.accounts.program_state, amount)?;

    emit_tokens_burned(ctx.accounts.owner.key(), amount)?;

    msg!(
        "Burned {} tokens from {}. Previous balance: {}, New balance: {}, Total burned: {}",
        amount,
        ctx.accounts.owner.key(),
        token_balance,
        new_balance,
        ctx.accounts.program_state.total_burned
    );

    Ok(())
}

fn validate_burn_amount(amount: u64) -> Result<()> {
    require!(amount > 0, SolLearningError::InvalidAmount);
    Ok(())
}

fn burn_tokens(ctx: &Context<BurnInstruction>, amount: u64) -> Result<()> {
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
    Ok(())
}

fn update_burned_tokens(program_state: &mut Account<ProgramState>, amount: u64) -> Result<()> {
    program_state.total_burned = program_state
        .total_burned
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;
    Ok(())
}

fn emit_tokens_burned(burner: Pubkey, amount: u64) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    emit!(TokensBurned {
        burner,
        amount,
        timestamp,
    });

    Ok(())
}
