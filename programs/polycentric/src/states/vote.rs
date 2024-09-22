use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub governance_pool: Pubkey,
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub approve: bool,
    pub timestamp: i64,
}

impl Vote {
    pub const LEN: usize = 8 + // discriminator
        32 + // governance_pool
        32 + // proposal
        32 + // voter
        1 + // approve
        8; // timestamp

}