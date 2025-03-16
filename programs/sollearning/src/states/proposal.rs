use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub multisig: Pubkey,
    pub index: u64,
    pub instruction: ProposalInstruction,
    pub signers: Vec<bool>,
    pub status: ProposalStatus,
    pub created_at: i64,
    pub closed_at: Option<i64>,
    pub description: String,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ProposalInstruction {
    ChangeAuthority { new_authority: Pubkey },
    TogglePause { paused: bool },
    RegisterEducator { educator: Pubkey, mint_limit: u64 },
    UpdateEducatorStatus { educator: Pubkey, is_active: bool, mint_limit: Option<u64> },
    AddSigner { signer: Pubkey },
    RemoveSigner { signer: Pubkey },
    ChangeThreshold { threshold: u8 },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,
    Executed,
    Cancelled,
    Expired,
}

#[event]
pub struct ProposalCreated {
    pub multisig: Pubkey,
    pub proposal: Pubkey,
    pub proposer: Pubkey,
    pub instruction: ProposalInstruction,
    pub description: String,
    pub timestamp: i64,
}

#[event]
pub struct ProposalApproved {
    pub multisig: Pubkey,
    pub proposal: Pubkey,
    pub signer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProposalExecuted {
    pub multisig: Pubkey,
    pub proposal: Pubkey,
    pub executor: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProposalCancelled {
    pub multisig: Pubkey,
    pub proposal: Pubkey,
    pub canceller: Pubkey,
    pub timestamp: i64,
}
