use anchor_lang::prelude::*;
use anchor_spl::token::{self};
use crate::error::SolLearningError;
use crate::instructions::structs::transfer_struct::TransferInstruction;
use crate::utils::pause::{check_program_running, check_function_running};
use crate::constants::PAUSE_FLAG_TRANSFER;

pub fn transfer_handler(ctx: Context<TransferInstruction>, amount: u64) -> Result<()> {
    check_program_running(&ctx.accounts.program_state)?;
    check_function_running(&ctx.accounts.program_state, PAUSE_FLAG_TRANSFER)?;
    
    validate_transfer_amount(amount)?;
    
    let from_balance = token::accessor::amount(&ctx.accounts.from)?;
    let to_balance = token::accessor::amount(&ctx.accounts.to)?;

    validate_sender_balance(from_balance, amount)?;

    execute_transfer(&ctx, amount)?;

    log_transfer(&ctx, amount, from_balance, to_balance)?;

    Ok(())
}

fn validate_transfer_amount(amount: u64) -> Result<()> {
    require!(amount > 0, SolLearningError::InvalidAmount);
    Ok(())
}

fn validate_sender_balance(from_balance: u64, amount: u64) -> Result<()> {
    let buffer_amount = amount / 10;
    require!(
        from_balance >= amount.checked_add(buffer_amount).ok_or(SolLearningError::Overflow)?,
        SolLearningError::TransferFrontRunning
    );
    Ok(())
}

fn execute_transfer(ctx: &Context<TransferInstruction>, amount: u64) -> Result<()> {
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
    Ok(())
}

fn log_transfer(ctx: &Context<TransferInstruction>, amount: u64, from_balance: u64, to_balance: u64) -> Result<()> {
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