use anchor_lang::prelude::*;

use crate::constants::*;
use crate::states::Deposit;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct User {
    pub user: Pubkey,
    pub voting_power: u64,
    pub points: u64,
    pub created_at: i64,
    pub deposits: Vec<Deposit>,
}

impl User {
    pub const LEN: usize = PUBLIC_KEY_LENGTH
        + 8
        + TIMESTAMP_LENGTH
        + VECTOR_LENGTH_PREFIX;

    pub fn total_user_deposit_amount(&self) -> u64 {
        self.deposits.iter().map(|deposit| {
            if !deposit.deactivating {deposit.amount} else {0u64}
        }).sum()
    }
}