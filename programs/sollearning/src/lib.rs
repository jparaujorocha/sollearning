use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod states;
pub mod instructions;
pub mod utils;

use crate::states::proposal::ProposalInstruction;

// Import handlers
use instructions::educator::*;
use instructions::student::*;
use instructions::course::*;
use instructions::proposal::*;
use instructions::multisig::*;
use instructions::emergency::*;
use instructions::token::*;
use instructions::config::*;
use instructions::initialize::*;

// Import structs
use instructions::proposal::approve_proposal_struct::ApproveProposal;
use instructions::token::burn_struct::BurnInstruction;
use instructions::course::create_course_struct::CreateCourse;
use instructions::multisig::create_multisig_struct::CreateMultisig;
use instructions::proposal::create_proposal_struct::CreateProposal;
use instructions::student::create_student_account_struct::CreateStudentTokenAccount;
use instructions::emergency::emergency_toggle_struct::EmergencyToggle;
use instructions::proposal::execute_proposal_struct::ExecuteProposal;
use instructions::initialize::initialize_struct::InitializeAccounts;
use instructions::token::mint_struct::MintToStudent;
use instructions::educator::register_educator_struct::RegisterEducator;
use instructions::student::register_student_struct::RegisterStudent;
use instructions::educator::set_educator_status_struct::SetEducatorStatus;
use instructions::token::transfer_struct::TransferInstruction;
use instructions::course::update_course_struct::UpdateCourse;
use instructions::config::create_program_config_struct::CreateProgramConfig;
use instructions::config::update_program_config_struct::UpdateProgramConfig;
use instructions::multisig::create_emergency_multisig_struct::CreateEmergencyMultisig;

// Import _client_accounts_ (mantido conforme solicitado)
use instructions::proposal::create_proposal_struct::__client_accounts_create_proposal;
use instructions::proposal::approve_proposal_struct::__client_accounts_approve_proposal;
use instructions::proposal::execute_proposal_struct::__client_accounts_execute_proposal;
use instructions::token::mint_struct::__client_accounts_mint_to_student;
use instructions::token::burn_struct::__client_accounts_burn_instruction;
use instructions::token::transfer_struct::__client_accounts_transfer_instruction;
use instructions::student::create_student_account_struct::__client_accounts_create_student_token_account;
use instructions::student::register_student_struct::__client_accounts_register_student;
use instructions::educator::register_educator_struct::__client_accounts_register_educator;
use instructions::educator::set_educator_status_struct::__client_accounts_set_educator_status;
use instructions::multisig::create_multisig_struct::__client_accounts_create_multisig;
use instructions::multisig::create_emergency_multisig_struct::__client_accounts_create_emergency_multisig;
use instructions::emergency::emergency_toggle_struct::__client_accounts_emergency_toggle;
use instructions::course::create_course_struct::__client_accounts_create_course;
use instructions::course::update_course_struct::__client_accounts_update_course;
use instructions::config::create_program_config_struct::__client_accounts_create_program_config;
use instructions::config::update_program_config_struct::__client_accounts_update_program_config;
use instructions::initialize::initialize_struct::__client_accounts_initialize_accounts;


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

    pub fn approve_proposal(ctx: Context<ApproveProposal>) -> Result<()> {
        approve_proposal_handler(ctx)
    }
    
    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        execute_proposal_handler(ctx)
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, instruction: ProposalInstruction, description: String) -> Result<()> {
        create_proposal_handler(ctx, instruction, description)
    }

    pub fn create_multisig(ctx: Context<CreateMultisig>, signers: Vec<Pubkey>, threshold: u8) -> Result<()> {
        create_multisig_handler(ctx, signers, threshold)
    }

    pub fn create_program_config(
        ctx: Context<CreateProgramConfig>,
        max_educators: u16,
        max_courses_per_educator: u16,
        max_mint_amount: u64,
        mint_cooldown_period: i64,
    ) -> Result<()> {
        create_program_config_handler(
            ctx,
            max_educators,
            max_courses_per_educator,
            max_mint_amount,
            mint_cooldown_period,
        )
    }
    
    pub fn update_program_config(
        ctx: Context<UpdateProgramConfig>,
        max_educators: Option<u16>,
        max_courses_per_educator: Option<u16>,
        max_mint_amount: Option<u64>,
        mint_cooldown_period: Option<i64>,
    ) -> Result<()> {
        update_program_config_handler(
            ctx,
            max_educators,
            max_courses_per_educator,
            max_mint_amount,
            mint_cooldown_period,
        )
    }
    
    pub fn create_emergency_multisig(
        ctx: Context<CreateEmergencyMultisig>,
        signers: Vec<Pubkey>,
        threshold: u8
    ) -> Result<()> {
        create_emergency_multisig_handler(ctx, signers, threshold)
    }
}