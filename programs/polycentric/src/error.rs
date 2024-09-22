use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Maximum number of assemblies reached")]
    MaxAssembliesReached,
    #[msg("Maximum number of policy areas reached")]
    MaxPolicyAreasReached,
    #[msg("Maximum number of assembly members reached")]
    MaxAssemblyMembersReached,
    #[msg("Maximum number of policy areas per assembly reached")]
    MaxPolicyAreasPerAssemblyReached,
    #[msg("Maximum number of assemblies per policy area reached")]
    MaxAssembliesPerPolicyAreaReached,
    #[msg("Maximum number of proposals per policy area reached")]
    MaxProposalsPerPolicyAreaReached,
    #[msg("Proposal is not in active state")]
    ProposalNotActive,
    #[msg("Proposal voting period has ended")]
    ProposalVotingEnded,
    #[msg("User has already voted on this proposal")]
    AlreadyVoted,
    #[msg("User is not a member of the required assembly")]
    NotAssemblyMember,
    #[msg("Proposal is not approved")]
    ProposalNotApproved,
    #[msg("Insufficient permissions to perform this action")]
    InsufficientPermissions,
    #[msg("Invalid governance pool")]
    InvalidGovernancePool,
    #[msg("Invalid assembly")]
    InvalidAssembly,
    #[msg("Invalid policy area")]
    InvalidPolicyArea,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Maximum treasuries reached")]
    MaxTreasuriesReached
}