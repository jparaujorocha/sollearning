use {
    anchor_lang::prelude::*,
    solana_program_test::*,
    solana_sdk::{
        signature::Signer,
        transaction::Transaction,
    },
    sollearning::{
        state::{
            EducatorAccount, 
            StudentInfo, 
            Course, 
            ProgramState, 
            CourseCompletion
        },
        error::SolLearningError,
    }
};

/// Test context to simulate Solana program environment
pub struct TestContext {
    pub program_test: ProgramTest,
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub authority: Keypair,
}

/// Setup test environment with default configurations
pub async fn setup_test() -> TestContext {
    let mut program_test = ProgramTest::new(
        "sollearning", 
        sollearning::ID, 
        None
    );

    let authority = Keypair::new();
    program_test.add_account(
        authority.pubkey(), 
        Account::new(10_000_000_000, 0, &system_program::ID)
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    TestContext {
        program_test,
        banks_client,
        payer,
        recent_blockhash,
        authority,
    }
}

/// Helper function to register an educator
pub async fn register_educator(
    ctx: &mut TestContext, 
    educator_pubkey: Pubkey
) -> (Pubkey, u8) {
    let (educator_pda, bump) = Pubkey::find_program_address(
        &[b"educator", educator_pubkey.as_ref()],
        sollearning::ID
    );

    let ix = sollearning::instruction::register_educator(
        ctx.authority.pubkey(),
        educator_pubkey,
        educator_pda,
        1_000_000 // Default minting limit
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer, &ctx.authority],
        ctx.recent_blockhash
    );

    ctx.banks_client.process_transaction(tx).await
        .expect("Educator registration should succeed");

    (educator_pda, bump)
}

/// Helper function to register a student
pub async fn register_student(
    ctx: &mut TestContext, 
    student_pubkey: Pubkey
) -> (Pubkey, u8) {
    let (student_pda, bump) = Pubkey::find_program_address(
        &[b"student", student_pubkey.as_ref()],
        sollearning::ID
    );

    let ix = sollearning::instruction::register_student(
        student_pubkey,
        student_pda
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer, &student_pubkey],
        ctx.recent_blockhash
    );

    ctx.banks_client.process_transaction(tx).await
        .expect("Student registration should succeed");

    (student_pda, bump)
}

/// Helper function to create a course
pub async fn create_course(
    ctx: &mut TestContext,
    educator_pubkey: Pubkey,
    course_id: &str
) -> (Pubkey, u8) {
    let (course_pda, bump) = Pubkey::find_program_address(
        &[
            b"course", 
            educator_pubkey.as_ref(), 
            course_id.as_bytes()
        ],
        sollearning::ID
    );

    let ix = sollearning::instruction::create_course(
        educator_pubkey,
        course_id.to_string(),
        "Test Course".to_string(),
        100, // Reward amount
        [0; 32] // Metadata hash
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer, &educator_pubkey],
        ctx.recent_blockhash
    );

    ctx.banks_client.process_transaction(tx).await
        .expect("Course creation should succeed");

    (course_pda, bump)
}

/// Helper function to mint tokens to a student
pub async fn mint_tokens_to_student(
    ctx: &mut TestContext,
    educator_pubkey: Pubkey,
    student_pubkey: Pubkey,
    course_pubkey: Pubkey,
    course_id: &str,
    amount: u64
) -> Pubkey {
    let (course_completion_pda, _) = Pubkey::find_program_address(
        &[
            b"course-completion", 
            student_pubkey.as_ref(), 
            course_id.as_bytes()
        ],
        sollearning::ID
    );

    let ix = sollearning::instruction::mint_to_student(
        educator_pubkey,
        student_pubkey,
        course_pubkey,
        course_id.to_string(),
        amount
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer, &educator_pubkey, &student_pubkey],
        ctx.recent_blockhash
    );

    ctx.banks_client.process_transaction(tx).await
        .expect("Token minting should succeed");

    course_completion_pda
}

/// Helper function to transfer tokens
pub async fn transfer_tokens(
    ctx: &mut TestContext,
    sender_pubkey: Pubkey,
    recipient_pubkey: Pubkey,
    amount: u64
) {
    let ix = sollearning::instruction::transfer(
        sender_pubkey,
        recipient_pubkey,
        amount
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer, &sender_pubkey],
        ctx.recent_blockhash
    );

    ctx.banks_client.process_transaction(tx).await
        .expect("Token transfer should succeed");
}

/// Helper function to burn tokens
pub async fn burn_tokens(
    ctx: &mut TestContext,
    owner_pubkey: Pubkey,
    amount: u64
) {
    let ix = sollearning::instruction::burn(
        owner_pubkey,
        amount
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer, &owner_pubkey],
        ctx.recent_blockhash
    );

    ctx.banks_client.process_transaction(tx).await
        .expect("Token burn should succeed");
}