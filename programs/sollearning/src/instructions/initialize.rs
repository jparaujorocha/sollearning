use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint};
use crate::states::program::ProgramState;
use crate::states::config::ProgramConfig;
use crate::states::course::TokensMinted;
use crate::constants::*;
use crate::instructions::structs::initialize_struct::InitializeAccounts;

pub fn initialize_handler(ctx: Context<InitializeAccounts>) -> Result<()> {
    let program_bump = ctx.bumps.program_state;
    let config_bump = ctx.bumps.program_config;
    let current_time = Clock::get()?.unix_timestamp;

    initialize_program_state(
        &mut ctx.accounts.program_state,
        &ctx.accounts.token_mint,
        &ctx.accounts.authority,
        program_bump
    )?;

    initialize_program_config(
        &mut ctx.accounts.program_config,
        &ctx.accounts.authority,
        current_time,
        config_bump
    )?;

    create_authority_token_account(&ctx)?;

    mint_initial_supply(&ctx, program_bump)?;

    update_total_minted(&mut ctx.accounts.program_state)?;

    emit_tokens_minted(&ctx)?;

    log_initialization(&ctx);

    Ok(())
}

fn initialize_program_state(
    program_state: &mut Account<ProgramState>,
    token_mint: &Account<Mint>,
    authority: &Signer,
    bump: u8,
) -> Result<()> {
    program_state.token_mint = token_mint.key();
    program_state.authority = authority.key();
    program_state.total_minted = 0;
    program_state.total_burned = 0;
    program_state.educator_count = 0;
    program_state.paused = false;
    program_state.pause_flags = 0;
    program_state.bump = bump;
    Ok(())
}

fn initialize_program_config(
    program_config: &mut Account<ProgramConfig>,
    authority: &Signer,
    current_time: i64,
    bump: u8,
) -> Result<()> {
    program_config.max_educators = MAX_EDUCATORS_LIMIT;
    program_config.max_courses_per_educator = MAX_COURSES_PER_EDUCATOR;
    program_config.max_mint_amount = MAX_MINT_AMOUNT;
    program_config.mint_cooldown_period = MINT_COOLDOWN_PERIOD;
    program_config.authority = authority.key();
    program_config.last_updated_at = current_time;
    program_config.bump = bump;
    Ok(())
}

fn create_authority_token_account(ctx: &Context<InitializeAccounts>) -> Result<()> {
    anchor_spl::associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.authority.to_account_info(),
                associated_token: ctx.accounts.authority_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ),
    )?;
    Ok(())
}

fn mint_initial_supply(ctx: &Context<InitializeAccounts>, bump: u8) -> Result<()> {
    let signer_seeds = &[PROGRAM_STATE_SEED, &[bump]];
    let signer = &[&signer_seeds[..]];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.authority_token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            signer,
        ),
        INITIAL_SUPPLY,
    )?;
    Ok(())
}

fn update_total_minted(program_state: &mut Account<ProgramState>) -> Result<()> {
    program_state.total_minted = INITIAL_SUPPLY;
    Ok(())
}

fn emit_tokens_minted(ctx: &Context<InitializeAccounts>) -> Result<()> {
    emit!(TokensMinted {
        recipient: ctx.accounts.authority.key(),
        amount: INITIAL_SUPPLY,
        minted_by: ctx.accounts.authority.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

fn log_initialization(ctx: &Context<InitializeAccounts>) {
    msg!("{} token initialized with {} tokens", TOKEN_NAME, INITIAL_SUPPLY);
    msg!("Token mint: {}", ctx.accounts.token_mint.key());
    msg!("Token symbol: {}", TOKEN_SYMBOL);
    msg!("Authority token account: {}", ctx.accounts.authority_token_account.key());
}