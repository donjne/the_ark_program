use anchor_lang::prelude::*;

#[error_code]
pub enum AbsoluteMonarchyError {
    #[msg("Only the Monarch can perform this action")]
    NotMonarch,
    #[msg("Insufficient funds in the royal treasury")]
    InsufficientFunds,
    #[msg("Subject not found")]
    SubjectNotFound,
    #[msg("Monarchy already initialized")]
    AlreadyInitialized,
    #[msg("Invalid decree")]
    InvalidDecree,
    #[msg("War already declared against this enemy")]
    WarAlreadyDeclared,
    #[msg("Subject is not convicted")]
    SubjectNotConvicted,
    #[msg("Invalid noble title")]
    InvalidNobleTitle,
    #[msg("Territory already colonized")]
    TerritoryAlreadyColonized,
    #[msg("Invalid economic policy")]
    InvalidEconomicPolicy,
    #[msg("Invalid tax rate")]
    InvalidTaxRate,
    #[msg("Policy title is too long")]
    PolicyTitleTooLong,
    #[msg("Policy description is too long")]
    PolicyDescriptionTooLong,
    #[msg("Jurisdiction name is too long")]
    JurisdictionTooLong,
    #[msg("Not the policy owner")]
    NotPolicyOwner,
    #[msg("Invalid usage fee rate")]
    InvalidUsageFeeRate,
    #[msg("Invalid access level")]
    InvalidAccessLevel,
    #[msg("Privileged access has expired")]
    AccessExpired,
    #[msg("Transfer is not yet due")]
    TransferTooSoon,
    #[msg("Wrong treasury")]
    InvalidTreasury


}