use anchor_lang::prelude::*;

#[account]
pub struct Kingdom {
    pub name: String,
    pub monarch: Pubkey,
    pub established_at: i64,
}

impl Kingdom {
    pub const MAXIMUM_NAME_LENGTH: usize = 100;

    pub fn space() -> usize {
        8 +  // discriminator
        4 + Self::MAXIMUM_NAME_LENGTH + // name (String)
        32 + // monarch (Pubkey)
        8    // established_at (i64)
    }
}