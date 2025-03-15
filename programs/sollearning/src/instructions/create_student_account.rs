use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::associated_token::AssociatedToken;
use crate::state::ProgramState;
use crate::constants::*;
use crate::error::SolLearningError; 

#[derive(Accounts)]
pub struct CreateStudentTokenAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub program_state: Account<'info, ProgramState>,
    /// CHECK: Usado como autoridade para a nova conta de token
    #[account(
        address = program_state.token_mint,
    )]
    pub token_mint: AccountInfo<'info>,
    
    /// CHECK: Verificamos que este mint corresponde ao configurado no estado do programa
    pub student: UncheckedAccount<'info>,
    /// CHECK: Verificamos que este mint corresponde ao configurado no estado do programa
    #[account(mut)]
    pub student_token_account: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_student_token_account_handler(ctx: Context<CreateStudentTokenAccount>) -> Result<()> {
    anchor_spl::associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.student_token_account.to_account_info(),
                authority: ctx.accounts.student.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ),
    )?;

    msg!(
        "Created token account for student: {}",
        ctx.accounts.student.key()
    );

    msg!(
        "Token account address: {}",
        ctx.accounts.student_token_account.key()
    );

    Ok(())
}
