use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};
use crate::{state::ProgramState, constants::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = program_state,
    )]
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<ProgramState>(),
        seeds = [b"program-state".as_ref()],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        init,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = authority,
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;
    
    // Set program state values
    program_state.token_mint = ctx.accounts.token_mint.key();
    program_state.authority = ctx.accounts.authority.key();
    program_state.total_minted = INITIAL_SUPPLY;
    program_state.total_burned = 0;
    program_state.bump = ctx.bumps.program_state;
    
    // Mint initial supply to the authority
    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.authority_token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            &[&[
                b"program-state".as_ref(),
                &[ctx.bumps.program_state],
            ]],
        ),
        INITIAL_SUPPLY,
    )?;
    
    msg!("SolLearning token initialized with {} tokens", INITIAL_SUPPLY);
    msg!("Token mint: {}", ctx.accounts.token_mint.key());
    
    Ok(())
}