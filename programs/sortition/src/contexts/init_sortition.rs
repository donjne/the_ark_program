use anchor_lang::prelude::*;
use crate::states::governance::{GovernancePool, DemographicQuotas};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct InitializeGovernance<'info> {
    #[account(init, payer = admin, space = GovernancePool::space())]
    pub governance_pool: Account<'info, GovernancePool>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_governance(
    ctx: Context<InitializeGovernance>, 
    assembly_size: u8,
    regions: [u32; 10],
    age_groups: [u32; 5],
    other_demographic: [u32; 3],
) -> Result<()> {
    require!(assembly_size > 0 && assembly_size <= 100, GovernanceError::InvalidAssemblySize);
    
    // Validate that quotas sum up to assembly_size
    let total_regions = regions.iter().sum::<u32>();
    let total_age_groups = age_groups.iter().sum::<u32>();
    let total_other_demographic = other_demographic.iter().sum::<u32>();

    let total_quota = total_regions.max(total_age_groups).max(total_other_demographic);
    require!(total_quota == assembly_size as u32, GovernanceError::InvalidQuotas);

    let governance_pool = &mut ctx.accounts.governance_pool;
    governance_pool.admin = ctx.accounts.admin.key();
    governance_pool.total_citizens = 0;
    governance_pool.assembly_size = assembly_size;
    governance_pool.assembly_term = 0;
    governance_pool.current_assembly = Pubkey::default();
    governance_pool.last_random_seed = [0; 32];
    governance_pool.selection_in_progress = false;
    governance_pool.total_citizen_indices = 0;
    governance_pool.demographic_quotas = DemographicQuotas {
        regions,
        age_groups,
        other_demographic,
    };
    
    Ok(())
}