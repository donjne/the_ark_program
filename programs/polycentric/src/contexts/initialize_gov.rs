use anchor_lang::prelude::*;
use crate::states::{GovernancePool, Analytics};
use crate::error::GovernanceError;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String, description: String)]
pub struct InitializeGovernment<'info> {
    #[account(
        init,
        payer = admin,
        space = GovernancePool::LEN,
        seeds = [b"governance_pool", admin.key().as_ref()],
        bump
    )]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(
        init_if_needed,
        payer = admin,
        space = Analytics::LEN,
        seeds = [b"analytics"],
        bump
    )]
    pub analytics: Account<'info, Analytics>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<InitializeGovernment>, name: String, description: String) -> Result<()> {
    require!(name.len() <= MAX_NAME_LENGTH, GovernanceError::InvalidGovernancePool);
    require!(description.len() <= MAX_DESCRIPTION_LENGTH, GovernanceError::InvalidGovernancePool);

    let governance_pool = &mut ctx.accounts.governance_pool;
    let analytics = &mut ctx.accounts.analytics;
    let clock = &ctx.accounts.clock;

    // pub name: String,
    // pub description: String,
    // pub admin: Pubkey,
    // pub assemblies: Vec<Pubkey>,
    // pub policy_areas: Vec<Pubkey>,
    // pub treasuries: Vec<Pubkey>,
    // pub total_participants: u64,
    // pub total_proposals: u64,
    // pub total_votes: u64,
    
        governance_pool.name = name;
        governance_pool.description = description;
        governance_pool.admin = ctx.accounts.admin.key();
        governance_pool.assemblies = Vec::new();
        governance_pool.policy_areas = Vec::new();
        governance_pool.treasuries = Vec::new();
        governance_pool.total_participants = 0;
        governance_pool.total_proposals = 0;
        governance_pool.total_votes = 0;

    // Update analytics
    analytics.increment_governance_pools();
    analytics.update_timestamp(clock.unix_timestamp);

    Ok(())
}