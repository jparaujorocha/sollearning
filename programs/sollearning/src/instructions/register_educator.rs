use anchor_lang::prelude::*;
use crate::states::program::ProgramState;
use crate::states::educator::EducatorAccount;
use crate::constants::*;
use crate::error::SolLearningError;

#[derive(Accounts)]
#[instruction(mint_limit: u64)]
pub struct RegisterEducator<'info> {
    #[account(
        mut,
        constraint = authority.key() == program_state.authority @ SolLearningError::Unauthorized,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub authority: Signer<'info>,

    #[account(
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
    )]
    pub program_state: Account<'info, ProgramState>,
    /// CHECK: Usado apenas como referência para derivar o PDA da conta do educador
    pub educator: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<EducatorAccount>(),
        seeds = [EDUCATOR_SEED, educator.key().as_ref()],
        bump,
    )]
    pub educator_account: Account<'info, EducatorAccount>,

    pub system_program: Program<'info, System>,
}

pub fn register_educator_handler(
    ctx: Context<RegisterEducator>,
    mint_limit: u64,
) -> Result<()> {
    require!(mint_limit > 0 && mint_limit <= MAX_MINT_AMOUNT, SolLearningError::InvalidAmount);

    let educator_account = &mut ctx.accounts.educator_account;
    educator_account.educator_address = ctx.accounts.educator.key();
    educator_account.authority = ctx.accounts.authority.key();
    educator_account.mint_limit = mint_limit;
    educator_account.total_minted = 0;
    educator_account.is_active = true;
    educator_account.bump = ctx.bumps.educator_account;

    msg!(
        "Registered educator {} with mint limit of {}",
        ctx.accounts.educator.key(),
        mint_limit
    );

    Ok(())
}
