use anchor_lang::prelude::*;

#[account]
pub struct Officer {
    pub authority: Pubkey,
    pub rank: u8,
    pub appointed_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Rank {
    General = 0,
    Colonel = 1,
    Major = 2,
    Captain = 3,
}