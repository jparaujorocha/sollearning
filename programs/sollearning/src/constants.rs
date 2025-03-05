pub const TOKEN_NAME: &str = "SolLearning";
pub const TOKEN_SYMBOL: &str = "SLEARNING";
pub const TOKEN_DECIMALS: u8 = 9;
pub const INITIAL_SUPPLY: u64 = 100_000_000_000_000_000; // 100M tokens with 9 decimals

// Seeds for PDAs
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";
pub const EDUCATOR_SEED: &[u8] = b"educator";
pub const STUDENT_SEED: &[u8] = b"student";
pub const ADMIN_SEED: &[u8] = b"admin";

// Various constants for the program
pub const MAX_MINT_AMOUNT: u64 = 1_000_000_000_000_000; // 1M tokens with 9 decimals