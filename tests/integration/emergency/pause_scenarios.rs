#[cfg(test)]
mod emergency_pause_tests {
    use super::*;
    use crate::common::setup_test;
    use sollearning::{
        error::SolLearningError,
        constants::{
            PAUSE_FLAG_MINT, 
            PAUSE_FLAG_TRANSFER, 
            PAUSE_FLAG_ALL
        }
    };

    #[tokio::test]
    async fn emergency_total_program_pause() {
        let mut ctx = setup_test().await;

        let pause_ix = sollearning::instruction::emergency_toggle(
            ctx.authority.pubkey(),
            true // Pause
        );

        let tx = Transaction::new_signed_with_payer(
            &[pause_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ctx.authority],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx).await
            .expect("Program pause should succeed");

        // Verify program state
        let program_state_pda = Pubkey::find_program_address(
            &[b"program-state"],
            sollearning::ID
        ).0;

        let program_state_account = ctx.banks_client
            .get_account(program_state_pda)
            .await
            .expect("Program state account must exist");

        let program_state = ProgramState::try_deserialize(
            &mut program_state_account.data.as_slice()
        ).expect("Must deserialize");

        assert!(program_state.paused);
        assert_eq!(program_state.pause_flags, PAUSE_FLAG_ALL);
    }

    #[tokio::test]
    async fn emergency_granular_pause() {
        let mut ctx = setup_test().await;

        let pause_ix = sollearning::instruction::emergency_toggle_granular(
            ctx.authority.pubkey(),
            PAUSE_FLAG_MINT | PAUSE_FLAG_TRANSFER, // Pause minting and transfers
            true
        );

        let tx = Transaction::new_signed_with_payer(
            &[pause_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &ctx.authority],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx).await
            .expect("Granular pause should succeed");

        // Verify program state
        let program_state_pda = Pubkey::find_program_address(
            &[b"program-state"],
            sollearning::ID
        ).0;

        let program_state_account = ctx.banks_client
            .get_account(program_state_pda)
            .await
            .expect("Program state account must exist");

        let program_state = ProgramState::try_deserialize(
            &mut program_state_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(program_state.pause_flags, PAUSE_FLAG_MINT | PAUSE_FLAG_TRANSFER);
    }

    #[tokio::test]
    async fn emergency_pause_by_unauthorized_authority() {
        let mut ctx = setup_test().await;

        let unauthorized_authority = Keypair::new();

        let pause_ix = sollearning::instruction::emergency_toggle(
            unauthorized_authority.pubkey(),
            true // Pause
        );

        let tx = Transaction::new_signed_with_payer(
            &[pause_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &unauthorized_authority],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Unauthorized pause should fail");
    }
}