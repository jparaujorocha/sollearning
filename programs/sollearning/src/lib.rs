use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod states;
pub mod instructions;


use crate::states::proposal::ProposalInstruction;

// Import handlers
use instructions::initialize::initialize_handler;
use instructions::register_educator::register_educator_handler;
use instructions::set_educator_status::set_educator_status_handler;
use instructions::register_student::register_student_handler;
use instructions::create_student_account::create_student_token_account_handler;
use instructions::create_course::create_course_handler;
use instructions::update_course::update_course_handler;
use instructions::mint::mint_to_student_handler;
use instructions::transfer::transfer_handler;
use instructions::burn::burn_handler;
use instructions::emergency_toggle::emergency_toggle_handler;
use instructions::approve_proposal::approve_proposal_handler;
use instructions::execute_proposal::execute_proposal_handler;
use instructions::create_multisig::create_multisig_handler;
use instructions::create_proposal::create_proposal_handler;

// Import structs from `structs/`
use instructions::structs::approve_proposal_struct::ApproveProposal;
use instructions::structs::burn_struct::BurnInstruction;
use instructions::structs::create_course_struct::CreateCourse;
use instructions::structs::create_multisig_struct::CreateMultisig;
use instructions::structs::create_proposal_struct::CreateProposal;
use instructions::structs::create_student_account_struct::CreateStudentTokenAccount;
use instructions::structs::emergency_toggle_struct::EmergencyToggle;
use instructions::structs::execute_proposal_struct::ExecuteProposal;
use instructions::structs::initialize_struct::InitializeAccounts;
use instructions::structs::mint_struct::MintToStudent;
use instructions::structs::register_educator_struct::RegisterEducator;
use instructions::structs::register_student_struct::RegisterStudent;
use instructions::structs::set_educator_status_struct::SetEducatorStatus;
use instructions::structs::transfer_struct::TransferInstruction;
use instructions::structs::update_course_struct::UpdateCourse;

// Import _client_accounts_ from structs (generated by Anchor)
use instructions::structs::approve_proposal_struct::__client_accounts_approve_proposal;
use instructions::structs::burn_struct::__client_accounts_burn_instruction;
use instructions::structs::create_course_struct::__client_accounts_create_course;
use instructions::structs::create_multisig_struct::__client_accounts_create_multisig;
use instructions::structs::create_proposal_struct::__client_accounts_create_proposal;
use instructions::structs::create_student_account_struct::__client_accounts_create_student_token_account;
use instructions::structs::emergency_toggle_struct::__client_accounts_emergency_toggle;
use instructions::structs::execute_proposal_struct::__client_accounts_execute_proposal;
use instructions::structs::initialize_struct::__client_accounts_initialize_accounts;
use instructions::structs::mint_struct::__client_accounts_mint_to_student;
use instructions::structs::register_educator_struct::__client_accounts_register_educator;
use instructions::structs::register_student_struct::__client_accounts_register_student;
use instructions::structs::set_educator_status_struct::__client_accounts_set_educator_status;
use instructions::structs::transfer_struct::__client_accounts_transfer_instruction;
use instructions::structs::update_course_struct::__client_accounts_update_course;

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
}