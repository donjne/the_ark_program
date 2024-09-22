use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::GovernanceError;

#[account]
pub struct Proposal {
    pub governance_pool: Pubkey,
    pub policy_area: Pubkey,
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub status: ProposalStatus,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
    Executed,
}

impl Proposal {
    pub const LEN: usize = 8 + // discriminator
        32 + // governance_pool
        32 + // policy_area
        32 + // creator
        4 + MAX_TITLE_LENGTH + // title
        4 + MAX_DESCRIPTION_LENGTH + // description
        1 + // status (enum)
        8 + // yes_votes
        8 + // no_votes
        8 + // start_time
        8; // end_time

    pub fn vote(&mut self, approve: bool) {
        if approve {
            self.yes_votes += 1;
        } else {
            self.no_votes += 1;
        }
    }

    pub fn finalize(&mut self) {
        if self.yes_votes > self.no_votes {
            self.status = ProposalStatus::Approved;
        } else {
            self.status = ProposalStatus::Rejected;
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        require!(self.status == ProposalStatus::Approved, GovernanceError::ProposalNotApproved);

        self.status = ProposalStatus::Executed;

        Ok(())
    }
}