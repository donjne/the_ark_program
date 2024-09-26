use anchor_lang::prelude::*;
use reclaim::cpi::accounts::VerifyProof;
use reclaim::cpi::verify_proof;
use reclaim::instructions::VerifyProofArgs;
use reclaim::program::Reclaim;
use reclaim::state::ClaimData as ReclaimClaimData;
use reclaim::state::ClaimInfo as ReclaimClaimInfo;
use reclaim::state::SignedClaim as ReclaimSignedClaim;
use reclaim::state::{Epoch, EpochConfig};
use crate::errors::RouterError;


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VerifyArgs {
    pub claim_info: ClaimInfo,
    pub signed_claim: SignedClaim,
}

/// Represents a signed claim
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SignedClaim {
    pub claim_data: ClaimData,
    pub signatures: Vec<[u8; 65]>,
}

/// Information about the claim
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimInfo {
    pub provider: String,
    pub parameters: String,
    pub context_address: Pubkey,
    pub context_message: String,
}

/// Data of the claim
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimData {
    pub identifier: [u8; 32],
    pub owner: String,
    pub timestamp: u32,
    pub epoch_index: u32,
}

#[derive(Accounts)]
pub struct Verify<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub epoch_config: Account<'info, EpochConfig>,
    pub epoch: Account<'info, Epoch>,
    pub reclaim_program: Program<'info, Reclaim>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 64 + 8 + 1, // Adjust space as needed
        seeds = [b"verification", signer.key().as_ref()],
        bump
    )]
    pub verification_result: Account<'info, VerificationResult>,
}

/// Accounts required for listing verifications
#[derive(Accounts)]
pub struct ListVerifications<'info> {
    pub signer: Signer<'info>,
}

/// Accounts required for revoking a verification
#[derive(Accounts)]
pub struct RevokeVerification<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"verification", signer.key().as_ref()],
        bump,
        constraint = verification_result.signer == signer.key() @ RouterError::Unauthorized
    )]
    pub verification_result: Account<'info, VerificationResult>,
}

/// Struct to store verification results
#[account]
pub struct VerificationResult {
    pub signer: Pubkey,
    pub provider: String,
    pub verified_at: i64,
    pub is_active: bool,
}


/// Verify a claim using the Reclaim protocol
pub fn verify(ctx: Context<Verify>, args: VerifyArgs) -> Result<()> {
    let VerifyArgs {
        claim_info,
        signed_claim,
    } = args;

    // Signer and rent payer
    let signer_account_info = ctx.accounts.signer.to_account_info();
    // Program Account infos
    let reclaim_program_info = ctx.accounts.reclaim_program.to_account_info();
    
    // Proof verification
    let epoch_config_account_info = ctx.accounts.epoch_config.to_account_info();
    let epoch_account_info = ctx.accounts.epoch.to_account_info();

    // Perform the verification using Reclaim's CPI
    verify_proof(
        CpiContext::new(
            reclaim_program_info,
            VerifyProof {
                epoch_config: epoch_config_account_info,
                epoch: epoch_account_info,
                signer: signer_account_info,
            },
        ),
        VerifyProofArgs {
            claim_info: ReclaimClaimInfo {
                parameters: claim_info.parameters,
                context_message: claim_info.context_message,
                provider: claim_info.provider.clone(),
                context_address: claim_info.context_address,
            },
            signed_claim: ReclaimSignedClaim {
                claim_data: ReclaimClaimData {
                    epoch_index: signed_claim.claim_data.epoch_index,
                    timestamp: signed_claim.claim_data.timestamp,
                    identifier: signed_claim.claim_data.identifier,
                    owner: signed_claim.claim_data.owner,
                },
                signatures: signed_claim.signatures,
            },
        },
    )?;

    // Store the verification result
    let verification_result = &mut ctx.accounts.verification_result;
    verification_result.signer = ctx.accounts.signer.key();
    verification_result.provider = claim_info.provider;
    verification_result.verified_at = Clock::get()?.unix_timestamp;
    verification_result.is_active = true;

    // Emit the verification event
    emit!(VerificationCompleteEvent {
        signer: ctx.accounts.signer.key(),
        provider: verification_result.provider.clone(),
        success: true,
    });

    Ok(())
}

/// List all verifications for a specific signer
pub fn list_verifications(ctx: Context<ListVerifications>) -> Result<()> {
    // The actual listing is done client-side by fetching VerificationResult accounts
    // So this particular instruction is a no-op. It is just used as a namespace for fetching accounts
    Ok(())
}

/// Revoke a specific verification
pub fn revoke_verification(ctx: Context<RevokeVerification>) -> Result<()> {
    let verification_result = &mut ctx.accounts.verification_result;
        
    require!(
        verification_result.is_active,
        RouterError::AlreadyRevoked
    );

    verification_result.is_active = false;

    emit!(VerificationRevokedEvent {
        signer: verification_result.signer,
        provider: verification_result.provider.clone(),
    });

    Ok(())
}

/// The event emitted when a verification is completed
#[event]
pub struct VerificationCompleteEvent {
    pub signer: Pubkey,
    pub provider: String,
    pub success: bool,
}

/// The event emitted when a verification is revoked
#[event]
pub struct VerificationRevokedEvent {
    pub signer: Pubkey,
    pub provider: String,
}