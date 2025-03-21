pub const TOKEN_NAME: &str = "SolLearning";
pub const TOKEN_SYMBOL: &str = "SLEARNING";
pub const TOKEN_DECIMALS: u8 = 9;
pub const INITIAL_SUPPLY: u64 = 100_000_000_000_000_000; // 100M tokens with 9 decimals

// Seeds for PDAs
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";
pub const PROGRAM_STATE_SEED: &[u8] = b"program-state";
pub const EDUCATOR_SEED: &[u8] = b"educator";
pub const STUDENT_SEED: &[u8] = b"student";
pub const ADMIN_SEED: &[u8] = b"admin";
pub const COURSE_SEED: &[u8] = b"course";
pub const COURSE_COMPLETION_SEED: &[u8] = b"course-completion";
pub const MULTISIG_SEED: &[u8] = b"multisig";
pub const PROPOSAL_SEED: &[u8] = b"proposal";
pub const COURSE_HISTORY_SEED: &[u8] = b"course-history";
pub const CONFIG_SEED: &[u8] = b"program-config";
pub const EMERGENCY_MULTISIG_SEED: &[u8] = b"emergency-multisig";

// Various constants for the program
pub const MAX_MINT_AMOUNT: u64 = 1_000_000_000_000_000; // 1M tokens with 9 decimals
pub const MAX_COURSE_ID_LENGTH: usize = 50;
pub const MAX_COURSE_NAME_LENGTH: usize = 100;
pub const MAX_EDUCATORS_LIMIT: u16 = 1000; // Maximum number of educators allowed
pub const MAX_COURSES_PER_EDUCATOR: u16 = 100; // Maximum courses per educator
pub const MAX_SIGNERS: usize = 10; // Maximum number of signers in multisig
pub const MAX_DESCRIPTION_LENGTH: usize = 200; // Maximum length for proposal descriptions
pub const MAX_CHANGE_DESCRIPTION_LENGTH: usize = 200; // Maximum length for course change descriptions
pub const PROPOSAL_EXPIRATION_TIME: i64 = 604800; // 7 days in seconds

// Time-based constants
pub const MINT_COOLDOWN_PERIOD: i64 = 7200; // 2 hours in seconds

// Function pause flags
pub const PAUSE_FLAG_MINT: u32 = 1 << 0;
pub const PAUSE_FLAG_TRANSFER: u32 = 1 << 1;
pub const PAUSE_FLAG_BURN: u32 = 1 << 2;
pub const PAUSE_FLAG_REGISTER: u32 = 1 << 3;
pub const PAUSE_FLAG_COURSE: u32 = 1 << 4;
pub const PAUSE_FLAG_ALL: u32 = 0xFFFFFFFF;