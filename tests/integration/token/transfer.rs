#[cfg(test)]
mod token_transfer_tests {
    use super::*;
    use crate::common::{
        setup_test, 
        register_educator, 
        register_student,
        create_course,
        mint_tokens_to_student,
        transfer_tokens
    };
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_token_transfer() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let sender = Keypair::new();
        let _ = register_student(&mut ctx, sender.pubkey()).await;

        let recipient = Keypair::new();
        let _ = register_student(&mut ctx, recipient.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let transfer_amount = 50;
        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            sender.pubkey(), 
            course_pda, 
            course_id, 
            100
        ).await;

        transfer_tokens(
            &mut ctx, 
            sender.pubkey(), 
            recipient.pubkey(), 
            transfer_amount
        ).await;

        // Additional verification logic would go here
    }

    #[tokio::test]
    async fn transfer_exceeding_balance() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let sender = Keypair::new();
        let _ = register_student(&mut ctx, sender.pubkey()).await;

        let recipient = Keypair::new();
        let _ = register_student(&mut ctx, recipient.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            sender.pubkey(), 
            course_pda, 
            course_id, 
            100
        ).await;

        let transfer_ix = sollearning::instruction::transfer(
            sender.pubkey(),
            recipient.pubkey(),
            150 // More than sender's balance
        );

        let tx = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &sender, &recipient],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Transfer exceeding balance should fail");
    }

    #[tokio::test]
    async fn transfer_to_unregistered_recipient() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let _ = register_educator(&mut ctx, educator.pubkey()).await;

        let sender = Keypair::new();
        let _ = register_student(&mut ctx, sender.pubkey()).await;

        let unregistered_recipient = Keypair::new();

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let _course_completion_pda = mint_tokens_to_student(
            &mut ctx, 
            educator.pubkey(), 
            sender.pubkey(), 
            course_pda, 
            course_id, 
            100
        ).await;

        let transfer_ix = sollearning::instruction::transfer(
            sender.pubkey(),
            unregistered_recipient.pubkey(),
            50
        );

        let tx = Transaction::new_signed_with_payer(
            &[transfer_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &sender, &unregistered_recipient],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Transfer to unregistered recipient should fail");
    }
}