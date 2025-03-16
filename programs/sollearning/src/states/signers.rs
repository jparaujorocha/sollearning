use anchor_lang::prelude::*;

#[account]
pub struct Multisig {
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub proposal_count: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

#[event]
pub struct MultisigCreated {
    pub multisig: Pubkey,
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub timestamp: i64,
}
