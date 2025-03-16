#[cfg(test)]
mod proposal_governance_tests {
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
        ).await
            .expect("Multisig creation should succeed");

        (ctx, signers, multisig_pda)
    }

    #[tokio::test]
    async fn execute_change_authority_proposal() {
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

        // Approve proposal by required signers
        let approve_proposal_ix1 = sollearning::instruction::approve_proposal(
            signers[1].pubkey(),
            proposal_pda,
            multisig_pda
        );

        let approve_proposal_ix2 = sollearning::instruction::approve_proposal(
            signers[2].pubkey(),
            proposal_pda,
            multisig_pda
        );

        let tx2 = Transaction::new_signed_with_payer(
            &[approve_proposal_ix1, approve_proposal_ix2],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[1], &signers[2]],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx2).await
            .expect("Proposal approvals should succeed");

        // Execute proposal
        let execute_proposal_ix = sollearning::instruction::execute_proposal(
            signers[0].pubkey(),
            proposal_pda,
            multisig_pda
        );

        let tx3 = Transaction::new_signed_with_payer(
            &[execute_proposal_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[0]],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx3).await
            .expect("Proposal execution should succeed");

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

        assert_eq!(program_state.authority, new_authority, "Authority should be updated");

        // Verify proposal state
        let proposal_account = ctx.banks_client
            .get_account(proposal_pda)
            .await
            .expect("Proposal account must exist");

        let proposal_data = Proposal::try_deserialize(
            &mut proposal_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(proposal_data.status, ProposalStatus::Executed);
    }

    #[tokio::test]
    async fn execute_proposal_with_insufficient_approvals() {
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

        // Approve by only one signer (insufficient)
        let approve_proposal_ix = sollearning::instruction::approve_proposal(
            signers[1].pubkey(),
            proposal_pda,
            multisig_pda
        );

        let tx2 = Transaction::new_signed_with_payer(
            &[approve_proposal_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[1]],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx2).await
            .expect("Single approval should succeed");

        // Try to execute proposal with insufficient approvals
        let execute_proposal_ix = sollearning::instruction::execute_proposal(
            signers[0].pubkey(),
            proposal_pda,
            multisig_pda
        );

        let tx3 = Transaction::new_signed_with_payer(
            &[execute_proposal_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &signers[0]],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx3).await;
        
        assert!(result.is_err(), "Proposal execution with insufficient approvals should fail");
    }
}