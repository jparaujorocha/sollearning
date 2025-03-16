#[cfg(test)]
mod course_management_tests {
    use super::*;
    use crate::common::{
        setup_test, 
        register_educator, 
        create_course
    };
    use sollearning::error::SolLearningError;

    #[tokio::test]
    async fn update_course_details() {
        let mut ctx = setup_test().await;
        
        let educator = Keypair::new();
        let (educator_pda, _) = register_educator(&mut ctx, educator.pubkey()).await;

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, educator.pubkey(), course_id).await;

        let update_ix = sollearning::instruction::update_course(
            educator.pubkey(),
            course_id.to_string(),
            Some("Updated Blockchain Course".to_string()),
            Some(200),
            Some(true),
            Some([1; 32]),
            "Course details update".to_string()
        );

        let tx = Transaction::new_signed_with_payer(
            &[update_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &educator],
            ctx.recent_blockhash
        );

        ctx.banks_client.process_transaction(tx).await
            .expect("Course update should succeed");

        let course_account = ctx.banks_client
            .get_account(course_pda)
            .await
            .expect("Course account must exist");

        let course_data = Course::try_deserialize(
            &mut course_account.data.as_slice()
        ).expect("Must deserialize");

        assert_eq!(course_data.course_name, "Updated Blockchain Course");
        assert_eq!(course_data.reward_amount, 200);
        assert!(course_data.is_active);
    }

    #[tokio::test]
    async fn update_course_by_unauthorized_educator() {
        let mut ctx = setup_test().await;
        
        let authorized_educator = Keypair::new();
        let _ = register_educator(&mut ctx, authorized_educator.pubkey()).await;

        let unauthorized_educator = Keypair::new();

        let course_id = "BLOCKCHAIN101";
        let (course_pda, _) = create_course(&mut ctx, authorized_educator.pubkey(), course_id).await;

        let update_ix = sollearning::instruction::update_course(
            unauthorized_educator.pubkey(), // Wrong educator
            course_id.to_string(),
            Some("Updated Course".to_string()),
            None,
            None,
            None,
            "Course update attempt".to_string()
        );

        let tx = Transaction::new_signed_with_payer(
            &[update_ix],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer, &unauthorized_educator],
            ctx.recent_blockhash
        );

        let result = ctx.banks_client.process_transaction(tx).await;
        
        assert!(result.is_err(), "Course update by unauthorized educator must fail");
    }
}