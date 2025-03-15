use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::{ProgramState, EducatorAccount, StudentInfo, Course, CourseCompletion, CourseCompleted, TokensMinted};
use crate::error::SolLearningError;
use crate::constants::*;

// Define minimum time between mintages to the same student (cooldown period)
const MIN_MINTAGE_INTERVAL: i64 = 3600; // 1 hour in seconds

#[derive(Accounts)]
#[instruction(amount: u64, course_id: String)]
pub struct MintToStudent<'info> {
    #[account(
        constraint = educator.is_active @ SolLearningError::InactiveEducator,
    )]
    pub educator: Account<'info, EducatorAccount>,
    
    #[account(
        mut,
        constraint = educator_authority.key() == educator.educator_address @ SolLearningError::Unauthorized,
    )]
    pub educator_authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [PROGRAM_STATE_SEED],
        bump = program_state.bump,
        constraint = !program_state.paused @ SolLearningError::ProgramPaused,
    )]
    pub program_state: Account<'info, ProgramState>,
    
    #[account(
        mut,
        address = program_state.token_mint,
    )]
    /// CHECK: We verify that this mint corresponds to the one configured in the program state
    pub token_mint: AccountInfo<'info>,
    
    #[account(
        mut,
        seeds = [STUDENT_SEED, student.key().as_ref()],
        bump = student_info.bump,
        // Add constraint to check cooldown period
        constraint = Clock::get()?.unix_timestamp - student_info.last_activity >= MIN_MINTAGE_INTERVAL
            @ SolLearningError::MintingTooFrequent,
    )]
    pub student_info: Account<'info, StudentInfo>,
    /// CHECK: Used only as a reference to derive the student_info PDA
    pub student: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = token::accessor::mint(&student_token_account)? == program_state.token_mint @ SolLearningError::InvalidMint,
        constraint = token::accessor::authority(&student_token_account)? == student.key() @ SolLearningError::Unauthorized,
    )]
    /// CHECK: We verify through constraints that this is a valid token account for the student
    pub student_token_account: AccountInfo<'info>,
    
    #[account(
        seeds = [COURSE_SEED, educator.key().as_ref(), course_id.as_bytes()],
        bump = course.bump,
        constraint = course.educator == educator.key() @ SolLearningError::CourseNotOwnedByEducator,
        constraint = course.is_active @ SolLearningError::CourseInactive,
    )]
    pub course: Account<'info, Course>,
    
    #[account(
        init,
        payer = educator_authority,
        space = 8 + 32 + 4 + course_id.len() + 32 + 8 + 8 + 1,
        seeds = [COURSE_COMPLETION_SEED, student.key().as_ref(), course_id.as_bytes()],
        bump,
    )]
    pub course_completion: Account<'info, CourseCompletion>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_to_student_handler(
    ctx: Context<MintToStudent>,
    amount: u64,
    course_id: String,
) -> Result<()> {
    // Validate the amount provided by the user
    require!(amount > 0 && amount <= ctx.accounts.educator.mint_limit, 
        SolLearningError::InvalidAmount);
    
    let current_time = Clock::get()?.unix_timestamp;
    
    // Get current student balances for improved logging
    let student_previous_balance = token::accessor::amount(&ctx.accounts.student_token_account)?;
    
    // Update educator stats
    let educator = &mut ctx.accounts.educator;
    educator.total_minted = educator.total_minted
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;
    
    // Update program state
    let program_state = &mut ctx.accounts.program_state;
    program_state.total_minted = program_state.total_minted
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;
    
    // Update student info
    let student_info = &mut ctx.accounts.student_info;
    student_info.total_earned = student_info.total_earned
        .checked_add(amount)
        .ok_or(SolLearningError::Overflow)?;
    student_info.courses_completed = student_info.courses_completed
        .checked_add(1)
        .ok_or(SolLearningError::Overflow)?;
    student_info.last_activity = current_time;
    
    // Encontre o bump manualmente para o CourseCompletion
    let (_, bump) = Pubkey::find_program_address(
        &[COURSE_COMPLETION_SEED, ctx.accounts.student.key().as_ref(), course_id.as_bytes()],
        ctx.program_id
    );
    
    // Update course completion
    let course_completion = &mut ctx.accounts.course_completion;
    course_completion.student = ctx.accounts.student.key();
    course_completion.course_id = course_id.clone();
    course_completion.verified_by = ctx.accounts.educator_authority.key();
    course_completion.completion_time = current_time;
    course_completion.tokens_awarded = amount;
    course_completion.bump = bump;
    
    // Update course stats
    let course = &ctx.accounts.course;
    let mut course_completion_count = course.completion_count;
    course_completion_count = course_completion_count
        .checked_add(1)
        .ok_or(SolLearningError::Overflow)?;
    
    // Prepare to mint tokens to student
    let program_state_seeds = &[PROGRAM_STATE_SEED, &[program_state.bump]];
    let program_state_signer = &[&program_state_seeds[..]];
    
    // Mint tokens to student
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.token_mint.to_account_info(),
                to: ctx.accounts.student_token_account.to_account_info(),
                authority: ctx.accounts.program_state.to_account_info(),
            },
            program_state_signer,
        ),
        amount,
    )?;
    
    // Calculate new balance for logging
    let student_new_balance = student_previous_balance + amount;
    
    // Emit completion event
    emit!(CourseCompleted {
        student: ctx.accounts.student.key(),
        course_id: course_id.clone(),
        educator: ctx.accounts.educator.key(),
        tokens_awarded: amount,
        timestamp: current_time,
    });
    
    // Emit token minting event
    emit!(TokensMinted {
        recipient: ctx.accounts.student.key(),
        amount,
        minted_by: ctx.accounts.educator_authority.key(),
        timestamp: current_time,
    });
    
    msg!(
        "Minted {} tokens to student {} for completing course '{}' ({}) - Previous balance: {}, New balance: {}",
        amount,
        ctx.accounts.student.key(),
        ctx.accounts.course.course_name,
        course_id,
        student_previous_balance,
        student_new_balance
    );
    
    Ok(())
}