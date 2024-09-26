use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub condition: String,
    pub is_fulfilled: bool,
    pub expiry_time: i64,
}