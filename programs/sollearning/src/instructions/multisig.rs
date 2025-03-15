use anchor_lang::prelude::*;
use crate::state::{MultisigState, ProgramState};
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
pub struct ApproveAdminAction<'info> {
    #[account(
        mut,
        seeds = [MULTISIG_SEED],
        bump = multisig.bump,
    )]
    pub multisig: Account<'info, MultisigState>,

    pub admin: Signer<'info>,
}

pub fn approve_admin_action(ctx: Context<ApproveAdminAction>, action: String) -> Result<()> {
    require!(ctx.accounts.multisig.admins.contains(&ctx.accounts.admin.key()), 
        SolLearningError::Unauthorized);
    
    let timestamp = Clock::get()?.unix_timestamp;
    
    emit!(AdminActionApproved {
        action: action.clone(),
        approved_by: ctx.accounts.admin.key(),
        timestamp,
    });

    msg!("Admin {} approved action: {}", ctx.accounts.admin.key(), action);

    Ok(())
}
