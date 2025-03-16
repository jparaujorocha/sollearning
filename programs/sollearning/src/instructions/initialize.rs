use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::program::ProgramState;
use crate::states::course::TokensMinted;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        seeds = [PROGRAM_STATE_SEED],
        bump,
        space = 8 + std::mem::size_of::<ProgramState>(),
    )]
    pub program_state: Account<'info, ProgramState>,
    
    /// CHECK: Used only as a reference for deriving the PDA
    #[account(
        init,
        payer = authority,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = program_state,
        mint::freeze_authority = program_state,
    )]
    pub token_mint: Account<'info, Mint>,

    /// CHECK: This account will be properly initialized during instruction execution
    #[account(mut)]
    pub authority_token_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_handler(ctx: Context<InitializeAccounts>) -> Result<()> {
    // Get bump value
    let (_, bump) = Pubkey::find_program_address(&[PROGRAM_STATE_SEED], ctx.program_id);
    
    // Fill program state data
    {
        let program_state = &mut ctx.accounts.program_state;
        program_state.token_mint = ctx.accounts.token_mint.key();
        program_state.authority = ctx.accounts.authority.key();
        program_state.total_minted = 0;
        program_state.total_burned = 0;
        program_state.educator_count = 0;
        program_state.paused = false;
        program_state.bump = bump;
    } // Mutable borrow ends here
    
    // Create the associated token account for authority
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

    // Mint initial supply to authority
    let signer_seeds = &[
        PROGRAM_STATE_SEED,
        &[bump],
    ];
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

    // Update total minted in program state
    {
        let program_state = &mut ctx.accounts.program_state;
        program_state.total_minted = INITIAL_SUPPLY;
    }
    
    // Get current timestamp for event
    let current_time = Clock::get()?.unix_timestamp;
    
    // Emit token minting event
    emit!(TokensMinted {
        recipient: ctx.accounts.authority.key(),
        amount: INITIAL_SUPPLY,
        minted_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });

    msg!(
        "{} token initialized with {} tokens",
        TOKEN_NAME,
        INITIAL_SUPPLY
    );

    msg!("Token mint: {}", ctx.accounts.token_mint.key());
    msg!("Token symbol: {}", TOKEN_SYMBOL);
    msg!("Authority token account: {}", ctx.accounts.authority_token_account.key());

    Ok(())
}