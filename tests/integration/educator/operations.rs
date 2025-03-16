#[cfg(test)]
mod educator_operations_tests {
    use super::*;
    use crate::common::{setup_test, register_educator};
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn update_educator_status() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        // Desativar educador
        let deactivate_ix = sollearning::instruction::set_educator_status(
            ctx.authority.pubkey(),
            educator.pubkey(),
            false,
            None
        );

        let tx = Transaction::new_signed_with_payer(
            &[deactivate_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ctx.authority],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx).await
            .expect("Educator deactivation should succeed");

        let educator_account = ctx.banks_client
            .get_account(educator_pda)
            .await
            .expect("Educator account must exist");

        let educator_data = EducatorAccount::try_deserialize(
            &mut educator_account.data.as_slice()
        ).expect("Must deserialize");

        assert!(!educator_data.is_active, "Educator should be inactive");
    }

    #[tokio::test]
    async fn update_educator_mint_limit() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        let new_mint_limit = 2_000_000;
        let update_limit_ix = sollearning::instruction::set_educator_status(
            ctx.authority.pubkey(),
            educator.pubkey(),
            true,
            Some(new_mint_limit)
        );

        let tx = Transaction::new_signed_with_payer(
            &[update_limit_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ctx.authority],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx).await
            .expect("Educator mint limit update should succeed");

        let educator_account = ctx.banks_client
            .get_account(educator_pda)
            .await
            .expect("Educator account must exist");

        let educator_data = EducatorAccount::try_deserialize(
            &mut educator_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(educator_data.mint_limit, new_mint_limit, "Mint limit should be updated");
    }
}