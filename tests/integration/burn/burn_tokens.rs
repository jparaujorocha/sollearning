#[cfg(test)]
mod burn_tokens_tests {
    use super::*;
    use crate::common::{
        setup_test, 
        register_educator, 
        register_student,
        create_course,
        mint_tokens_to_student,
        burn_tokens
    };
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_token_burn() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let student = Keypair::new();
        let _ = register_student(&mut ctx, student.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let burn_amount = 50;
        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            student.pubkey(), 
            course_pda, 
            course_id, 
            100
        ).await;

        burn_tokens(
            &mut ctx, 
            student.pubkey(), 
            burn_amount
        ).await;

        // Additional verification logic would go here
    }

    #[tokio::test]
    async fn burn_exceeding_balance() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let student = Keypair::new();
        let _ = register_student(&mut ctx, student.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            student.pubkey(), 
            course_pda, 
            course_id, 
            100
        ).await;

        let burn_ix = sollearning::instruction::burn(
            student.pubkey(),
            150 // More than student's balance
        );

        let tx = Transaction::new_signed_with_payer(
            &[burn_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &student],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Burn exceeding balance should fail");
    }

    #[tokio::test]
    async fn burn_zero_tokens() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let student = Keypair::new();
        let _ = register_student(&mut ctx, student.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            student.pubkey(), 
            course_pda, 
            course_id, 
            100
        ).await;

        let burn_ix = sollearning::instruction::burn(
            student.pubkey(),
            0 // Zero tokens
        );

        let tx = Transaction::new_signed_with_payer(
            &[burn_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &student],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Burn zero tokens should fail");
    }
}