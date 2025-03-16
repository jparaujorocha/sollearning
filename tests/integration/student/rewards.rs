#[cfg(test)]
mod student_rewards_tests {
    use super::*;
    use crate::common::{
        setup_test, 
        register_educator, 
        register_student, 
        create_course, 
        mint_tokens_to_student
    };
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_token_minting_for_student() {
        let mut ctx = setup_test().await;
        
        // Setup educator
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        // Setup student
        let student = Keypair::new();
        let (student_pda, _) = register_student(&mut ctx, student.pubkey()).await;

        // Create course
        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        // Mint tokens
        let reward_amount = 100;
        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            student.pubkey(), 
            course_pda, 
            course_id, 
            reward_amount
        ).await;

        // Verify student info
        let student_account = ctx.banks_client
            .get_account(student_pda)
            .await
            .expect("Student account must exist");

        let student_data = StudentInfo::try_deserialize(
            &mut student_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(student_data.total_earned, reward_amount);
        assert_eq!(student_data.courses_completed, 1);
    }

    #[tokio::test]
    async fn student_cannot_mint_without_course() {
        let mut ctx = setup_test().await;
        
        // Setup educator
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        // Setup student
        let student = Keypair::new();
        let (student_pda, _) = register_student(&mut ctx, student.pubkey()).await;

        // Create a non-existent course PDA
        let (course_pda, _) = Pubkey::find_program_address(
            &[
                b"course", 
                educator.pubkey().as_ref(), 
                b"NONEXISTENT_COURSE"
            ],
            sollearning::ID
        );

        let mint_ix = sollearning::instruction::mint_to_student(
            educator.pubkey(),
            student.pubkey(),
            course_pda,
            "NONEXISTENT_COURSE".to_string(),
            100
        );

        let tx = Transaction::new_signed_with_payer(
            &[mint_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &educator, &student],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Minting without valid course must fail");
    }

    #[tokio::test]
    async fn student_cannot_mint_from_unauthorized_educator() {
        let mut ctx = setup_test().await;
        
        // Setup authorized educator
        let authorized_educator = Keypair::new();
        let _ = register_educator(&mut ctx, authorized_educator.pubkey()).await;

        // Setup unauthorized educator
        let unauthorized_educator = Keypair::new();

        // Setup student
        let student = Keypair::new();
        let (student_pda, _) = register_student(&mut ctx, student.pubkey()).await;

        // Create course
        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, authorized_educator.pubkey(), course_id).await;

        let mint_ix = sollearning::instruction::mint_to_student(
            unauthorized_educator.pubkey(), // Wrong educator
            student.pubkey(),
            course_pda,
            course_id.to_string(),
            100
        );

        let tx = Transaction::new_signed_with_payer(
            &[mint_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &unauthorized_educator, &student],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Minting by unauthorized educator must fail");
    }
}