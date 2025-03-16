#[cfg(test)]
mod token_minting_tests {
    use super::*;
    use crate::common::{
        setup_test, 
        register_educator, 
        create_course,
        register_student,
        mint_tokens_to_student
    };
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_token_minting() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let student = Keypair::new();
        let (student_pda, _) = register_student(&mut ctx, student.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let reward_amount = 100;
        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            student.pubkey(), 
            course_pda, 
            course_id, 
            reward_amount
        ).await;

        let student_account = ctx.banks_client
            .get_account(student_pda)
            .await
            .expect("Student account must exist");

        let student_data = StudentInfo::try_deserialize(
            &mut student_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(student_data.total_earned, reward_amount);
    }

    #[tokio::test]
    async fn minting_exceeding_educator_limit() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        let student = Keypair::new();
        let _ = register_student(&mut ctx, student.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        // Attempt to mint more than educator's limit
        let excessive_amount = 2_000_000; // Assuming default limit is 1,000,000

        let mint_ix = sollearning::instruction::mint_to_student(
            educator.pubkey(),
            student.pubkey(),
            course_pda,
            course_id.to_string(),
            excessive_amount
        );

        let tx = Transaction::new_signed_with_payer(
            &[mint_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &educator, &student],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Minting beyond limit should fail");
    }

    #[tokio::test]
    async fn minting_to_unregistered_student() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let unregistered_student = Keypair::new();

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let mint_ix = sollearning::instruction::mint_to_student(
            educator.pubkey(),
            unregistered_student.pubkey(),
            course_pda,
            course_id.to_string(),
            100
        );

        let tx = Transaction::new_signed_with_payer(
            &[mint_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &educator, &unregistered_student],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Minting to unregistered student should fail");
    }
}