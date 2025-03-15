use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("6F7BtsWEyMSf3RxKxxCxyJEdWJDMiuBsMMpMEWvbJGtm");

#[program]
pub mod sollearning {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn register_educator(ctx: Context<RegisterEducator>, mint_limit: u64) -> Result<()> {
        register_educator_handler(ctx, mint_limit)
    }

    pub fn set_educator_status(ctx: Context<SetEducatorStatus>, is_active: bool) -> Result<()> {
        set_educator_status_handler(ctx, is_active)
    }

    pub fn register_student(ctx: Context<RegisterStudent>) -> Result<()> {
        register_student_handler(ctx)
    }

    pub fn create_student_token_account(ctx: Context<CreateStudentTokenAccount>) -> Result<()> {
        create_student_token_account_handler(ctx)
    }

    pub fn mint_to_student(ctx: Context<MintToStudent>, amount: u64, course_id: String) -> Result<()> {
        mint_to_student_handler(ctx, amount, course_id)
    }

    pub fn transfer(ctx: Context<TransferInstruction>, amount: u64) -> Result<()> {
        transfer_handler(ctx, amount)
    }

    pub fn burn(ctx: Context<BurnInstruction>, amount: u64) -> Result<()> {
        burn_handler(ctx, amount)
    }

    pub fn emergency_toggle(ctx: Context<EmergencyToggle>, paused: bool) -> Result<()> {
        emergency_toggle_handler(ctx, paused)
    }
}
