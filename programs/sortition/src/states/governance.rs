use anchor_lang::prelude::*;

#[account]
pub struct GovernancePool {
    pub admin: Pubkey,
    pub total_citizens: u32,
    pub assembly_size: u8,
    pub assembly_term: i64,
    pub current_assembly: Pubkey,
    pub last_random_seed: [u8; 32],
    pub selection_in_progress: bool,
    pub total_citizen_indices: u32,
    pub demographic_quotas: DemographicQuotas,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct DemographicQuotas {
    pub regions: [u32; 10],
    pub age_groups: [u32; 5],
    pub other_demographic: [u32; 3],
}

impl GovernancePool {
    pub fn space() -> usize {
        8 + // discriminator
        32 + // admin
        4 + // total_citizens
        1 + // assembly_size
        8 + // assembly_term
        32 + // current_assembly
        32 + // last_random_seed
        1 + // selection_in_progress
        4 + // total_citizen_indices
        (8 * 1) + (5 * 1) + (4 * 1) // demographic_quotas
    }
}

impl DemographicQuotas {
    pub fn reset(&mut self) {
        self.regions = [0; 10];
        self.age_groups = [0; 5];
        self.other_demographic = [0; 3];
    }
}