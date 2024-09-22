use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Citizen already registered")]
    CitizenAlreadyRegistered,
    #[msg("Assembly already active")]
    AssemblyAlreadyActive,
    #[msg("Not enough citizens for selection")]
    NotEnoughCitizens,
    #[msg("Not a member of the current assembly")]
    NotAssemblyMember,
    #[msg("Proposal already exists")]
    ProposalAlreadyExists,
    #[msg("Invalid vote")]
    InvalidVote,
    #[msg("Proposal not active")]
    ProposalNotActive,
    #[msg("Assembly term has ended")]
    AssemblyTermEnded,
    #[msg("Citizen not eligible")]
    CitizenNotEligible,
    #[msg("Invalid assembly size")]
    InvalidAssemblySize,
    #[msg("Invalid term length")]
    InvalidTermLength,
    #[msg("Random selection in progress")]
    RandomSelectionInProgress,
    #[msg("Invalid demographic quotas")]
    InvalidQuotas,
    #[msg("Invalid demographic information")]
    InvalidDemographic,
    #[msg("Citizen index is full")]
    CitizenIndexFull,
    #[msg("Invalid data")]
    InvalidInput
}