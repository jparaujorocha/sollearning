use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod states;
pub mod instructions;

// Importações explícitas dos módulos __client_accounts_* que o Anchor gera
use instructions::initialize::{ initialize_handler, InitializeAccounts, __client_accounts_initialize_accounts };
use instructions::register_educator::{ register_educator_handler, RegisterEducator, __client_accounts_register_educator };
use instructions::set_educator_status::{ set_educator_status_handler, SetEducatorStatus, __client_accounts_set_educator_status };
use instructions::register_student::{ register_student_handler, RegisterStudent, __client_accounts_register_student };
use instructions::create_student_account::{ create_student_token_account_handler, CreateStudentTokenAccount, __client_accounts_create_student_token_account };
use instructions::create_course::{ create_course_handler, CreateCourse, __client_accounts_create_course };
use instructions::update_course::{ update_course_handler, UpdateCourse, __client_accounts_update_course };
use instructions::mint::{ mint_to_student_handler, MintToStudent, __client_accounts_mint_to_student };
use instructions::transfer::{ transfer_handler, TransferInstruction, __client_accounts_transfer_instruction };
use instructions::burn::{ burn_handler, BurnInstruction, __client_accounts_burn_instruction };
use instructions::emergency_toggle::{ emergency_toggle_handler, EmergencyToggle, __client_accounts_emergency_toggle };

declare_id!("FpfRZaGL4sHYABnUxrr7U8956kH491RQTrHTwe9gGUAU");

#[program]
pub mod sollearning {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        initialize_handler(ctx)
    }

    pub fn register_educator(ctx: Context<RegisterEducator>, mint_limit: u64) -> Result<()> {
        register_educator_handler(ctx, mint_limit)
    }

    pub fn set_educator_status(
        ctx: Context<SetEducatorStatus>, 
        is_active: bool,
        new_mint_limit: Option<u64>,
    ) -> Result<()> {
        set_educator_status_handler(ctx, is_active, new_mint_limit)
    }

    pub fn register_student(ctx: Context<RegisterStudent>) -> Result<()> {
        register_student_handler(ctx)
    }

    pub fn create_student_token_account(ctx: Context<CreateStudentTokenAccount>) -> Result<()> {
        create_student_token_account_handler(ctx)
    }
    
    pub fn create_course(
        ctx: Context<CreateCourse>, 
        course_id: String, 
        course_name: String, 
        reward_amount: u64,
        metadata_hash: [u8; 32],
    ) -> Result<()> {
        create_course_handler(ctx, course_id, course_name, reward_amount, metadata_hash)
    }
    
    pub fn update_course(
        ctx: Context<UpdateCourse>,
        course_name: Option<String>,
        reward_amount: Option<u64>,
        is_active: Option<bool>,
        metadata_hash: Option<[u8; 32]>,
        change_description: String,
    ) -> Result<()> {
        update_course_handler(ctx, course_name, reward_amount, is_active, metadata_hash, change_description) 
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