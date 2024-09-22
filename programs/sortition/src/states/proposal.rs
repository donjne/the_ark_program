use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub assembly: Pubkey,
    pub proposer: Pubkey,
    pub description: String,
    pub votes: Vec<(Pubkey, bool)>, // (member, approval)
    pub status: ProposalStatus,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
}

impl Proposal {
    pub const MAX_DESCRIPTION_LENGTH: usize = 200;
    pub const MAX_VOTES: usize = 100;

    pub fn space() -> usize {
        8 + // discriminator
        32 + // assembly
        32 + // proposer
        4 + Self::MAX_DESCRIPTION_LENGTH + // description
        4 + (Self::MAX_VOTES * (32 + 1)) + // votes
        1 + // status
        8
    }
}