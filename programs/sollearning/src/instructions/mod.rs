// instructions/mod.rs
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

// Re-export instruction contexts
pub use initialize::InitializeAccounts;
pub use mint::MintToStudent;
pub use transfer::TransferInstruction;
pub use burn::BurnInstruction;
pub use register_educator::RegisterEducator;
pub use set_educator_status::SetEducatorStatus;
pub use register_student::RegisterStudent;
pub use create_student_account::CreateStudentTokenAccount;
pub use create_course::CreateCourse;
pub use update_course::UpdateCourse;
pub use emergency_toggle::EmergencyToggle;

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