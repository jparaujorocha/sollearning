#[cfg(test)]
mod course_creation_tests {
    use super::*;
    use crate::common::{setup_test, register_educator, create_course};
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_course_creation() {
        let mut ctx = setup_test().await;
        
        // Setup educator
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let course_account = ctx.banks_client
            .get_account(course_pda)
            .await
            .expect("Course account must exist");

        let course_data = Course::try_deserialize(
            &mut course_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(course_data.course_id, course_id);
        assert!(course_data.is_active);
        assert_eq!(course_data.reward_amount, 100);
    }

    #[tokio::test]
    async fn course_creation_with_invalid_parameters() {
        let test_cases = vec![
            ("", "Empty course ID"),
            ("A".repeat(51), "Course ID too long")
        ];

        for (invalid_course_id, description) in test_cases {
            let mut ctx = setup_test().await;
            
            let educator = Keypair::new();
            let _ = register_educator(&mut ctx, educator.pubkey()).await;

            let ix = sollearning::instruction::create_course(
                educator.pubkey(),
                invalid_course_id.to_string(),
                "Test Course".to_string(),
                100,
                [0; 32]
            );

            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer, &educator],
                ctx.recent_blockhash
            );

            let result = ctx.banks_client.process_transaction(tx).await;
            
            assert!(
                result.is_err(), 
                "Course creation with {} should fail", 
                description
            );
        }
    }

    #[tokio::test]
    async fn course_creation_by_unauthorized_educator() {
        let mut ctx = setup_test().await;
        
        let unauthorized_educator = Keypair::new();

        let course_id = "BLOCKCHAIN101";
        let ix = sollearning::instruction::create_course(
            unauthorized_educator.pubkey(), // Not registered
            course_id.to_string(),
            "Test Course".to_string(),
            100,
            [0; 32]
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &unauthorized_educator],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Course creation by unauthorized educator must fail");
    }
}