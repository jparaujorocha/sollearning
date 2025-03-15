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

// Various constants for the program
pub const MAX_MINT_AMOUNT: u64 = 1_000_000_000_000_000; // 1M tokens with 9 decimals
pub const MAX_COURSE_ID_LENGTH: usize = 50;
pub const MAX_COURSE_NAME_LENGTH: usize = 100;
pub const MAX_EDUCATORS_LIMIT: u16 = 1000; // Maximum number of educators allowed
pub const MAX_COURSES_PER_EDUCATOR: u16 = 100; // Maximum courses per educator