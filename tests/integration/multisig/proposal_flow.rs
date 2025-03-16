#[cfg(test)]
mod multisig_proposal_tests {
    use super::*;
    use crate::common::setup_test;
    use sollearning::{
        state::{ProposalInstruction, ProposalStatus},
        error::SolLearningError
    };

    async fn setup_multisig() -> (TestContext, Vec<Keypair>, Pubkey) {
        let mut ctx = setup_test().await;
        
        let signers = vec![
            Keypair::new(),
            Keypair::new(),
            Keypair::new()
        ];
        
        let threshold = 2;

        let (multisig_pda, _) = Pubkey::find_program_address(
            &[b"multisig"],
            sollearning::ID
        );

        let ix = sollearning::instruction::create_multisig(
            ctx.authority.pubkey(),
            signers.iter().map(|k| k.pubkey()).collect(),
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

        (ctx, signers, multisig_pda)
    }

    #[tokio::test]
    async fn create_and_approve_proposal() {
        let (mut ctx, signers, multisig_pda) = setup_multisig().await;

        let new_authority = Keypair::new().pubkey();
        let proposal_instruction = ProposalInstruction::ChangeAuthority { 
            new_authority 
        };

        let (proposal_pda, _) = Pubkey::find_program_address(
            &[
                b"proposal", 
                multisig_pda.as_ref(), 
                &0_u64.to_le_bytes()
            ],
            sollearning::ID
        );

        // Create proposal
        let create_proposal_ix = sollearning::instruction::create_proposal(
            signers[0].pubkey(),
            proposal_instruction.clone(),
            "Change program authority".to_string()
        );

        let tx1 = Transaction::new_signed_with_payer(
            &[create_proposal_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[0]],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx1).await
            .expect("Proposal creation should succeed");

        // Approve proposal
        let approve_proposal_ix = sollearning::instruction::approve_proposal();

        let tx2 = Transaction::new_signed_with_payer(
            &[approve_proposal_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[1]],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx2).await
            .expect("Proposal approval should succeed");

        // Verify proposal state
        let proposal_account = ctx.banks_client
            .get_account(proposal_pda)
            .await
            .expect("Proposal account must exist");

        let proposal_data = Proposal::try_deserialize(
            &mut proposal_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(proposal_data.status, ProposalStatus::Active);
        assert!(proposal_data.signers[0]); // First signer approved
        assert!(proposal_data.signers[1]); // Second signer approved
    }

    #[tokio::test]
    async fn proposal_expiration() {
        let (mut ctx, signers, multisig_pda) = setup_multisig().await;

        let new_authority = Keypair::new().pubkey();
        let proposal_instruction = ProposalInstruction::ChangeAuthority { 
            new_authority 
        };

        let (proposal_pda, _) = Pubkey::find_program_address(
            &[
                b"proposal", 
                multisig_pda.as_ref(), 
                &0_u64.to_le_bytes()
            ],
            sollearning::ID
        );

        // Create proposal
        let create_proposal_ix = sollearning::instruction::create_proposal(
            signers[0].pubkey(),
            proposal_instruction.clone(),
            "Change program authority".to_string()
        );

        let tx1 = Transaction::new_signed_with_payer(
            &[create_proposal_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[0]],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx1).await
            .expect("Proposal creation should succeed");

        // Simulate time passing (7 days + 1 second)
        let long_time = 7 * 24 * 60 * 60 + 1;
        ctx.recent_blockhash = hash(&long_time.to_le_bytes());

        // Try to approve expired proposal
        let approve_proposal_ix = sollearning::instruction::approve_proposal();

        let tx2 = Transaction::new_signed_with_payer(
            &[approve_proposal_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[1]],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx2).await;
        
        assert!(result.is_err(), "Expired proposal approval should fail");

        // Verify proposal state
        let proposal_account = ctx.banks_client
            .get_account(proposal_pda)
            .await
            .expect("Proposal account must exist");

        let proposal_data = Proposal::try_deserialize(
            &mut proposal_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(proposal_data.status, ProposalStatus::Expired);
    }
}