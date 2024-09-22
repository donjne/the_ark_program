use anchor_lang::prelude::*;

#[account]
pub struct CitizenIndex {
    pub governance_pool: Pubkey,
    pub citizens: Vec<Pubkey>,
    pub count: u32,
}

impl CitizenIndex {
    pub const MAX_CITIZENS_PER_INDEX: usize = 10;

    pub fn space() -> usize {
        8 + // discriminator
        32 + // governance_pool
        4 + (32 * Self::MAX_CITIZENS_PER_INDEX) + // citizens
        4 // count
    }

    pub fn find_citizen_account<'info>(&self, citizen_pubkey: &Pubkey, remaining_accounts: &[&'info AccountInfo<'info>]) -> Option<&'info AccountInfo<'info>> {
        if !self.citizens.contains(citizen_pubkey) {
            return None;
        }

        remaining_accounts.iter().find(|account_info| account_info.key == citizen_pubkey).copied()
    }
}