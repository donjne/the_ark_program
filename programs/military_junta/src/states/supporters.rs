use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

const MAX_SUPPORTERS: usize = 100; 
pub const SUPPORTERS_PER_LEVEL: usize = 10;

#[account]
pub struct Supporters {
    pub supporters: [Option<Pubkey>; MAX_SUPPORTERS],
    pub support_amounts: Vec<(Pubkey, u64)>,
    pub count: u8,
}

impl Supporters {
    pub fn add_supporter(&mut self, supporter: Pubkey) -> Result<()> {
        require!(self.count < MAX_SUPPORTERS as u8, ErrorCode::MaxSupportersReached);
        
        for slot in self.supporters.iter_mut() {
            if slot.is_none() {
                *slot = Some(supporter);
                self.count += 1;
                return Ok(());
            }
        }

        Err(ErrorCode::MaxSupportersReached.into())
    }

    pub fn is_supporter(&self, pubkey: &Pubkey) -> bool {
        self.supporters.iter().any(|s| s.as_ref() == Some(pubkey))
    }

    pub fn get_support_amount(&self, supporter: &Pubkey) -> u64 {
        self.support_amounts
            .iter()
            .find(|(pubkey, _)| pubkey == supporter)
            .map(|(_, amount)| *amount)
            .unwrap_or(0)
    }

    pub fn update_support_amount(&mut self, supporter: Pubkey, amount: u64) -> Result<()> {
        if let Some(entry) = self.support_amounts.iter_mut().find(|(pubkey, _)| *pubkey == supporter) {
            entry.1 = amount;
        } else {
            self.support_amounts.push((supporter, amount));
        }
        Ok(())
    }
}