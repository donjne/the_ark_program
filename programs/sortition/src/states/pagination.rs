use anchor_lang::prelude::*;
use crate::states::governance::DemographicQuotas;

#[account]
pub struct PaginationState {
    pub governance_pool: Pubkey,
    pub current_index: u32,
    pub current_citizen_in_index: u32,
    pub selected_citizens: Vec<Pubkey>,
    pub demographic_counts: DemographicQuotas,
}

impl PaginationState {
    pub fn space(max_assembly_size: usize) -> usize {
        8 + // discriminator
        32 + // governance_pool
        4 + // current_index
        4 + // current_citizen_in_index
        4 + (32 * max_assembly_size) + // selected_citizens
        (8 * 1) + (5 * 1) + (4 * 1) // demographic_counts
    }
}