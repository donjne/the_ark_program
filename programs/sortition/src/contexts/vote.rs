use anchor_lang::prelude::*;
use crate::states::{assembly::Assembly, proposal::{Proposal, ProposalStatus}};
use crate::error::GovernanceError;

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub assembly: Account<'info, Assembly>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub member: Signer<'info>,
}

pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, approve: bool) -> Result<()> {
    let assembly = &ctx.accounts.assembly;
    let proposal = &mut ctx.accounts.proposal;
    let member = &ctx.accounts.member;

    require!(assembly.members.contains(&member.key()), GovernanceError::NotAssemblyMember);
    require!(proposal.status == ProposalStatus::Active, GovernanceError::ProposalNotActive);
    require!(Clock::get()?.unix_timestamp < assembly.term_end, GovernanceError::AssemblyTermEnded);

    // Remove existing vote if any
    proposal.votes.retain(|(voter, _)| voter != &member.key());
    
    // Add new vote
    proposal.votes.push((member.key(), approve));

    // Check if all members have voted
    if proposal.votes.len() == assembly.members.len() {
        let approvals = proposal.votes.iter().filter(|(_, approve)| *approve).count();
        let threshold = assembly.members.len() * 2 / 3; // 2/3 majority

        if approvals > threshold {
            proposal.status = ProposalStatus::Passed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }
    }

    Ok(())
}