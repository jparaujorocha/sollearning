use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::ProgramState;
use crate::constants::*;
use solana_program::system_instruction;
use solana_program::program_pack::Pack;

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [crate::constants::PROGRAM_STATE_SEED],
        bump,
        space = 8 + std::mem::size_of::<ProgramState>(),
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub token_mint: Signer<'info>,

    #[account(mut)]
    pub authority_token_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_handler(ctx: Context<InitializeAccounts>) -> Result<()> {
    let rent = Rent::get()?;
    let mint_len = token::spl_token::state::Mint::LEN;
    let mint_rent = rent.minimum_balance(mint_len);

    let create_mint_ix = system_instruction::create_account(
        ctx.accounts.authority.key,
        ctx.accounts.token_mint.key,
        mint_rent,
        mint_len as u64,
        &token::ID
    );

    anchor_lang::solana_program::program::invoke_signed(
        &create_mint_ix,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_mint.to_account_info(),
        ],
        &[&[]]
    )?;

    token::initialize_mint(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::InitializeMint {
                mint: ctx.accounts.token_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        crate::constants::TOKEN_DECIMALS,
        ctx.accounts.authority.key,
        Some(ctx.accounts.authority.key),
    )?;

    let program_state = &mut ctx.accounts.program_state;
    program_state.token_mint = ctx.accounts.token_mint.key();
    program_state.authority = ctx.accounts.authority.key();
    program_state.total_minted = crate::constants::INITIAL_SUPPLY;
    program_state.total_burned = 0;
    program_state.paused = false;
    program_state.bump = ctx.bumps.program_state;

    msg!(
        "{} token initialized with {} tokens",
        crate::constants::TOKEN_NAME,
        crate::constants::INITIAL_SUPPLY
    );

    msg!("Token mint: {}", ctx.accounts.token_mint.key());
    msg!("Token symbol: {}", crate::constants::TOKEN_SYMBOL);

    Ok(())
}
