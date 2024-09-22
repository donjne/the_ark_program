use anchor_lang::error_code;


#[error_code]
pub enum GovernanceError {
    #[msg("Circle already exists")]
    CircleAlreadyExists,
    #[msg("Member already exists")]
    MemberAlreadyExists,
    #[msg("Proposal already exists")]
    ProposalAlreadyExists,
    #[msg("Invalid vote")]
    InvalidVote,
    #[msg("Not a circle member")]
    NotCircleMember,
    #[msg("Not authorized")]
    NotAuthorized,
    #[msg("Proposal not active")]
    ProposalNotActive,
}