use anchor_lang::prelude::*;
use crate::state::{Multisig, MultisigCreated};
use crate::error::SolLearningError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(signers: Vec<Pubkey>, threshold: u8)]
pub struct CreateMultisig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 4 + (signers.len() * 32) + 1 + 8 + 32 + 1,
        seeds = [MULTISIG_SEED],
        bump,
    )]
    pub multisig: Account<'info, Multisig>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_multisig_handler(
    ctx: Context<CreateMultisig>,
    signers: Vec<Pubkey>,
    threshold: u8,
) -> Result<()> {
    // Validate multisig configuration
    require!(!signers.is_empty(), SolLearningError::InvalidMultisigConfig);
    require!(signers.len() <= MAX_SIGNERS, SolLearningError::MaxSignersReached);
    require!(
        threshold > 0 && threshold as usize <= signers.len(),
        SolLearningError::InvalidThreshold
    );
    
    // Check for duplicate signers
    for (i, signer) in signers.iter().enumerate() {
        for (j, other_signer) in signers.iter().enumerate() {
            if i != j && signer == other_signer {
                return Err(SolLearningError::SignerAlreadyExists.into());
            }
        }
    }
    
    // Ensure the authority is one of the signers
    require!(
        signers.contains(&ctx.accounts.authority.key()),
        SolLearningError::Unauthorized
    );
    
    // Get bump
    let (_, bump) = Pubkey::find_program_address(&[MULTISIG_SEED], ctx.program_id);
    
    // Initialize multisig account
    let multisig = &mut ctx.accounts.multisig;
    multisig.signers = signers.clone();
    multisig.threshold = threshold;
    multisig.proposal_count = 0;
    multisig.authority = ctx.accounts.authority.key();
    multisig.bump = bump;
    
    // Emit multisig creation event
    let current_time = Clock::get()?.unix_timestamp;
    emit!(MultisigCreated {
        multisig: multisig.key(),
        signers: signers.clone(),
        threshold,
        timestamp: current_time,
    });
    
    msg!(
        "Created multisig with {} signers and threshold of {}",
        signers.len(),
        threshold
    );
    
    Ok(())
}