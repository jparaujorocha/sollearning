use anchor_lang::prelude::*;
use crate::constants::STUDENT_SEED;
use crate::states::program::ProgramState;
use crate::states::student::StudentInfo;

#[derive(Accounts)]
pub struct RegisterStudent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub student: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<StudentInfo>(),
        seeds = [STUDENT_SEED, student.key().as_ref()],
        bump,
    )]
    pub student_info: Account<'info, StudentInfo>,

    pub system_program: Program<'info, System>,
}
