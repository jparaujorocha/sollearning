#[cfg(test)]
mod multisig_creation_tests {
    use super::*;
    use crate::common::setup_test;
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_multisig_creation() {
        let mut ctx = setup_test().await;
        
        let signers = vec![
            Keypair::new().pubkey(),
            Keypair::new().pubkey(),
            Keypair::new().pubkey()
        ];
        
        let threshold = 2; // At least 2 signers must approve

        let (multisig_pda, _) = Pubkey::find_program_address(
            &[b"multisig"],
            sollearning::ID
        );

        let ix = sollearning::instruction::create_multisig(
            ctx.authority.pubkey(),
            signers.clone(),
            threshold
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ctx.authority],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx).await
            .expect("Multisig creation should succeed");

        let multisig_account = ctx.banks_client
            .get_account(multisig_pda)
            .await
            .expect("Multisig account must exist");

        let multisig_data = Multisig::try_deserialize(
            &mut multisig_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(multisig_data.signers, signers);
        assert_eq!(multisig_data.threshold, threshold);
        assert_eq!(multisig_data.proposal_count, 0);
    }

    #[tokio::test]
    async fn multisig_creation_with_invalid_parameters() {
        let test_cases = vec![
            (vec![], "Empty signers list"),
            (
                (0..12).map(|_| Keypair::new().pubkey()).collect(), 
                "Too many signers"
            )
        ];

        for (signers, description) in test_cases {
            let mut ctx = setup_test().await;
            
            let threshold = 2;

            let ix = sollearning::instruction::create_multisig(
                ctx.authority.pubkey(),
                signers.clone(),
                threshold
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
                "Multisig creation with {} should fail", 
                description
            );
        }
    }

    #[tokio::test]
    async fn multisig_creation_with_invalid_threshold() {
        let test_cases = vec![
            (0, "Zero threshold"),
            (11, "Threshold exceeding signers")
        ];

        for (threshold, description) in test_cases {
            let mut ctx = setup_test().await;
            
            let signers = vec![
                Keypair::new().pubkey(),
                Keypair::new().pubkey(),
                Keypair::new().pubkey()
            ];

            let ix = sollearning::instruction::create_multisig(
                ctx.authority.pubkey(),
                signers.clone(),
                threshold
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
                "Multisig creation with {} should fail", 
                description
            );
        }
    }
}