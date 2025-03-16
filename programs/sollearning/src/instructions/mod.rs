pub mod structs;

pub mod initialize;
pub mod mint;
pub mod transfer;
pub mod burn;
pub mod register_educator;
pub mod set_educator_status;
pub mod register_student;
pub mod create_student_account;
pub mod create_course;
pub mod update_course;
pub mod emergency_toggle;
pub mod approve_proposal;
pub mod execute_proposal;
pub mod create_multisig;
pub mod create_proposal;

// Re-export instruction contexts from the structs module
pub use structs::approve_proposal_struct::ApproveProposal;
pub use structs::burn_struct::BurnInstruction;
pub use structs::create_course_struct::CreateCourse;
pub use structs::create_multisig_struct::CreateMultisig;
pub use structs::create_proposal_struct::CreateProposal;
pub use structs::create_student_account_struct::CreateStudentTokenAccount;
pub use structs::emergency_toggle_struct::EmergencyToggle;
pub use structs::execute_proposal_struct::ExecuteProposal;
pub use structs::initialize_struct::InitializeAccounts;
pub use structs::mint_struct::MintToStudent;
pub use structs::register_educator_struct::RegisterEducator;
pub use structs::register_student_struct::RegisterStudent;
pub use structs::set_educator_status_struct::SetEducatorStatus;
pub use structs::transfer_struct::TransferInstruction;
pub use structs::update_course_struct::UpdateCourse;

// Re-export handler functions
pub use initialize::initialize_handler;
pub use mint::mint_to_student_handler;
pub use transfer::transfer_handler;
pub use burn::burn_handler;
pub use register_educator::register_educator_handler;
pub use set_educator_status::set_educator_status_handler;
pub use register_student::register_student_handler;
pub use create_student_account::create_student_token_account_handler;
pub use create_course::create_course_handler;
pub use update_course::update_course_handler;
pub use emergency_toggle::emergency_toggle_handler;
pub use approve_proposal::approve_proposal_handler;
pub use execute_proposal::execute_proposal_handler;
pub use create_multisig::create_multisig_handler;
pub use create_proposal::create_proposal_handler;
