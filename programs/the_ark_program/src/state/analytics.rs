// To count the number of instances that have been initialized from all governments and the number of 
// polls, approved proposals and disapproved proposal

use anchor_lang::prelude::*;
use crate::constants::*;


#[account]
pub struct ArkAnalytics {
    pub no_of_governments: u64,
    pub governments: Vec<Pubkey>,
    pub polls: u64,
    pub approved: u64,
    pub rejected: u64,
    pub points: u64,
    pub created_at: i64,
    pub auth_bump: u8,
    pub state_bump: u8,
}

impl ArkAnalytics {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // token, vault
        + 8 * 5 // daos, polls, approved, rejected, points 
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH * 2
        + VECTOR_LENGTH_PREFIX; // bump
}