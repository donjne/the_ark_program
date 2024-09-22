use anchor_lang::prelude::*;

#[account]
pub struct Circle {
    pub name: String,
    pub parent_circle: Option<Pubkey>,
    pub members: Vec<Pubkey>,
    pub proposals: Vec<Pubkey>,
}

impl Circle {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_MEMBERS: usize = 20;
    pub const MAX_PROPOSALS: usize = 50;

    pub fn space() -> usize {
        8 + // discriminator
        4 + Self::MAX_NAME_LENGTH + // name
        33 + // parent_circle (Option<Pubkey>)
        4 + (32 * Self::MAX_MEMBERS) + // members
        4 + (32 * Self::MAX_PROPOSALS) // proposals
    }
}