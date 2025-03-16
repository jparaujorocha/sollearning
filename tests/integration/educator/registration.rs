#[cfg(test)]
mod educator_registration_tests {
    use super::*;
    use crate::common::{setup_test, register_educator};
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_educator_registration() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        let educator_account = ctx.banks_client
            .get_account(educator_pda)
            .await
            .expect("Educator account must exist");

        let educator_data = EducatorAccount::try_deserialize(
            &mut educator_account.data.as_slice()
        ).expect("Must deserialize");

        assert!(educator_data.is_active);
        assert_eq!(educator_data.mint_limit, 1_000_000);
    }

    #[tokio::test]
    async fn educator_registration_with_invalid_limits() {
        let test_cases = vec![0, u64::MAX];

        for invalid_limit in test_cases {
            let mut ctx = setup_test().await;
            
            let educator = Keypair::new();
            let (educator_pda, _) = Pubkey::find_program_address(
                &[b"educator", educator.pubkey().as_ref()],
                sollearning::ID
            );

            let ix = sollearning::instruction::register_educator(
                ctx.authority.pubkey(),
                educator.pubkey(),
                educator_pda,
                invalid_limit
            );

            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer, &ctx.authority],
                ctx.recent_blockhash
            );

            let result = ctx.banks_client.process_transaction(tx).await;
            
            assert!(
                result.is_err(), 
                "Registration with limit {} must fail", 
                invalid_limit
            );
        }
    }

    #[tokio::test]
    async fn duplicate_educator_registration() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        let ix = sollearning::instruction::register_educator(
            ctx.authority.pubkey(),
            educator.pubkey(),
            educator_pda,
            1_000_000
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ctx.authority],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Duplicate registration must fail");
    }

    #[tokio::test]
    async fn unauthorized_educator_registration() {
        let mut ctx = setup_test().await;
        
        let unauthorized_authority = Keypair::new();
        let educator = Keypair::new();
        let (educator_pda, _) = Pubkey::find_program_address(
            &[b"educator", educator.pubkey().as_ref()],
            sollearning::ID
        );

        let ix = sollearning::instruction::register_educator(
            unauthorized_authority.pubkey(), // Wrong authority
            educator.pubkey(),
            educator_pda,
            1_000_000
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &unauthorized_authority],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Unauthorized registration must fail");
    }
}