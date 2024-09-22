use anchor_lang::prelude::*;
use crate::states::{GovernancePool, PolicyArea, Assembly, Proposal, ProposalStatus, Vote};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub governance_pool: Account<'info, GovernancePool>,
    
    #[account(mut)]
    pub policy_area: Account<'info, PolicyArea>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = voter,
        space = Vote::LEN,
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    #[account(
        constraint = assembly.governance_pool == governance_pool.key() 
        && assembly.policy_areas.contains(&policy_area.key())
    )]
    pub assembly: Account<'info, Assembly>,
    
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<CastVote>, approve: bool) -> Result<()> {
    let governance_pool = &mut ctx.accounts.governance_pool;
    let policy_area = &mut ctx.accounts.policy_area;
    let proposal = &mut ctx.accounts.proposal;
    let vote = &mut ctx.accounts.vote;
    let voter = &ctx.accounts.voter;
    let assembly = &mut ctx.accounts.assembly;
    let clock = &ctx.accounts.clock;

    // Ensure the voter is a member of the assembly
    require!(assembly.members.contains(&voter.key()), GovernanceError::NotAssemblyMember);

    // Ensure the proposal is still active
    require!(proposal.status == ProposalStatus::Active, GovernanceError::ProposalNotActive);

    // Ensure the voting period hasn't ended
    let current_time = clock.unix_timestamp;
    require!(current_time <= proposal.end_time, GovernanceError::ProposalVotingEnded);

    // Cast the vote
    proposal.vote(approve);

    // Create the vote record
    vote.governance_pool = governance_pool.key();
    vote.proposal = proposal.key();
    vote.voter = voter.key();
    vote.approve = approve;
    vote.timestamp = current_time;

    // Update counters
    governance_pool.increment_votes();
    policy_area.increment_votes();
    assembly.increment_votes();

    Ok(())
}