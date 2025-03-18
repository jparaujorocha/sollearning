use anchor_lang::prelude::*;
use crate::instructions::student::create_student_account_struct::CreateStudentTokenAccount;

pub fn create_student_token_account_handler(ctx: Context<CreateStudentTokenAccount>) -> Result<()> {
    create_associated_token_account(&ctx)?;
    log_token_creation(&ctx); 
    Ok(())
}

fn create_associated_token_account(ctx: &Context<CreateStudentTokenAccount>) -> Result<()> {
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
    Ok(())
}

fn log_token_creation(ctx: &Context<CreateStudentTokenAccount>) {
    msg!(
        "Created token account for student: {}",
        ctx.accounts.student.key()
    );

    msg!(
        "Token account address: {}",
        ctx.accounts.student_token_account.key()
    );
}
