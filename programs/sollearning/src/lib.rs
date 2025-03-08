use anchor_lang::prelude::*;
mod constants;
mod error;
pub mod instructions;
mod state;
use instructions::*;

declare_id!("6F7BtsWEyMSf3RxKxxCxyJEdWJDMiuBsMMpMEWvbJGtm");

#[program]
pub mod sollearning {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize_handler(ctx)
    }

    pub fn register_educator(ctx: Context<RegisterEducator>, mint_limit: u64) -> Result<()> {
        instructions::register_educator_handler(ctx, mint_limit)
    }

    pub fn register_student(ctx: Context<RegisterStudent>) -> Result<()> {
        instructions::register_student_handler(ctx)
    }

    pub fn create_student_token_account(ctx: Context<CreateStudentTokenAccount>) -> Result<()> {
        instructions::create_student_token_account_handler(ctx)
    }

    pub fn mint_to_student(ctx: Context<MintToStudent>, amount: u64, course_id: String) -> Result<()> {
        instructions::mint_to_student_handler(ctx, amount, course_id)
    }

    pub fn transfer(ctx: Context<TransferInstruction>, amount: u64) -> Result<()> {
        instructions::transfer_handler(ctx, amount)
    }

    pub fn burn(ctx: Context<BurnInstruction>, amount: u64) -> Result<()> {
        instructions::burn_handler(ctx, amount)
    }
}
