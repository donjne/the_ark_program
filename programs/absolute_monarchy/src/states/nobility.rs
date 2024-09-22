use anchor_lang::prelude::*;

#[account]
pub struct Noble {
    pub subject: Pubkey,
    pub title: String,
    pub granted_at: i64,
}

impl Noble {
    pub const MAXIMUM_TITLE_LENGTH: usize = 50;

    pub fn space() -> usize {
        32 + // subject
        4 + Self::MAXIMUM_TITLE_LENGTH + // title
        8 // granted_at
    }
}