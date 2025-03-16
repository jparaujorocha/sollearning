#[cfg(test)]
mod student_registration_tests {
    use super::*;
    use crate::common::{setup_test, register_student};
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn successful_student_registration() {
        let mut ctx = setup_test().await;
        
        let student = Keypair::new();
        let (student_pda, _) = register_student(&mut ctx, student.pubkey()).await;

        let student_account = ctx.banks_client
            .get_account(student_pda)
            .await
            .expect("Student account must exist");

        let student_data = StudentInfo::try_deserialize(
            &mut student_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(student_data.student_address, student.pubkey());
        assert_eq!(student_data.total_earned, 0);
        assert_eq!(student_data.courses_completed, 0);
    }

    #[tokio::test]
    async fn duplicate_student_registration() {
        let mut ctx = setup_test().await;
        
        let student = Keypair::new();
        let (student_pda, _) = register_student(&mut ctx, student.pubkey()).await;

        let ix = sollearning::instruction::register_student(
            student.pubkey(),
            student_pda
        );

        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &student],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Duplicate registration must fail");
    }
}