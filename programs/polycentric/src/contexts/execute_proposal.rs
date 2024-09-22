use anchor_lang::prelude::*;
use crate::states::{GovernancePool, PolicyArea, Assembly, Proposal, ProposalStatus};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(mut)]
    pub policy_area: Account<'info, PolicyArea>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub executor: Signer<'info>,
    
    #[account(
        constraint = assembly.governance_pool == governance_pool.key() 
        && assembly.policy_areas.contains(&policy_area.key())
    )]
    pub assembly: Account<'info, Assembly>,
    
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let executor = &ctx.accounts.executor;
    let assembly = &ctx.accounts.assembly;
    let clock = &ctx.accounts.clock;

    // Ensure the executor is a member of the assembly
    require!(assembly.members.contains(&executor.key()), GovernanceError::NotAssemblyMember);

    // Ensure the voting period has ended
    let current_time = clock.unix_timestamp;
    require!(current_time > proposal.end_time, GovernanceError::ProposalVotingEnded);

    // Finalize the proposal if it hasn't been done yet
    if proposal.status == ProposalStatus::Active {
        proposal.finalize();
    }

    // Execute the proposal
    proposal.execute()?;

    // Here, you would typically implement the actual execution logic
    // This could involve calling other instructions, transferring funds, etc.
    // For now, we'll just emit an event

    emit!(ProposalExecuted {
        governance_pool: ctx.accounts.governance_pool.key(),
        policy_area: ctx.accounts.policy_area.key(),
        proposal: proposal.key(),
        executor: executor.key(),
    });

    Ok(())
}

#[event]
pub struct ProposalExecuted {
    pub governance_pool: Pubkey,
    pub policy_area: Pubkey,
    pub proposal: Pubkey,
    pub executor: Pubkey,
}