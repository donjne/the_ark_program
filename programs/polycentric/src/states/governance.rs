use anchor_lang::prelude::*;
use crate::constants::*;
use crate::error::GovernanceError;

#[account]
pub struct GovernancePool {
    pub name: String,
    pub description: String,
    pub admin: Pubkey,
    pub assemblies: Vec<Pubkey>,
    pub policy_areas: Vec<Pubkey>,
    pub treasuries: Vec<Pubkey>,
    pub total_participants: u64,
    pub total_proposals: u64,
    pub total_votes: u64,
}
impl GovernancePool {

    pub const MAX_TREASURIES: usize = 5;

    pub const LEN: usize = 8 + // discriminator
        4 + MAX_NAME_LENGTH +  // name
        4 + MAX_DESCRIPTION_LENGTH + // description
        32 + // admin
        4 + (32 * MAX_ASSEMBLIES) + // assemblies
        4 + (32 * MAX_POLICY_AREAS) + // policy_areas
        4 + (32 * Self::MAX_TREASURIES) + // treasuries
        8 + // total_participants
        8 + // total_proposals
        8; // total_votes


    pub fn add_assembly(&mut self, assembly: Pubkey) -> Result<()> {
        require!(self.assemblies.len() < MAX_ASSEMBLIES, GovernanceError::MaxAssembliesReached);
        self.assemblies.push(assembly);
        Ok(())
    }

    pub fn add_policy_area(&mut self, policy_area: Pubkey) -> Result<()> {
        require!(self.policy_areas.len() < MAX_POLICY_AREAS, GovernanceError::MaxPolicyAreasReached);
        self.policy_areas.push(policy_area);
        Ok(())
    }

    pub fn increment_participants(&mut self) {
        self.total_participants += 1;
    }

    pub fn increment_proposals(&mut self) {
        self.total_proposals += 1;
    }

    pub fn increment_votes(&mut self) {
        self.total_votes += 1;
    }

    pub fn add_treasury(&mut self, treasury: Pubkey) -> Result<()> {
        require!(self.treasuries.len() < Self::MAX_TREASURIES, GovernanceError::MaxTreasuriesReached);
        self.treasuries.push(treasury);
        Ok(())
    }
}