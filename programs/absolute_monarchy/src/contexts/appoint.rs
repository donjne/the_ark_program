use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::states::{Monarch, Kingdom, Subject};
use crate::error::AbsoluteMonarchyError;
use anchor_spl::associated_token::AssociatedToken;


#[derive(Accounts)]
pub struct AppointOfficial<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(
        init,
        payer = authority,
        space = Subject::SPACE,
        seeds = [b"subject", kingdom.key().as_ref(), &kingdom.total_subjects.to_le_bytes()],
        bump
    )]
    pub subject: Box<Account<'info, Subject>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

pub fn appoint_official(ctx: Context<AppointOfficial>, role: String, jurisdiction: String) -> Result<()> {
    let subject_account = &mut ctx.accounts.subject;
    let kingdom = &mut ctx.accounts.kingdom;

    subject_account.key = subject_account.key();
    subject_account.role = role;
    subject_account.jurisdiction = jurisdiction;
    subject_account.loyalty = 100; // Start with full loyalty
    subject_account.wealth = 0;
    subject_account.is_convicted = false;
    subject_account.appointed_at = Clock::get()?.unix_timestamp;
    subject_account.bump = ctx.bumps.subject;

    kingdom.officials_appointed.push(subject_account.key());
    kingdom.total_subjects += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct UnappointOfficial<'info> {
    #[account(mut)]
    pub kingdom: Box<Account<'info, Kingdom>>,

    #[account(
        mut,
        has_one = authority @ AbsoluteMonarchyError::NotMonarch,
        constraint = monarch.key() == kingdom.monarch @ AbsoluteMonarchyError::MonarchKingdomMismatch
    )]
    pub monarch: Box<Account<'info, Monarch>>,

    #[account(mut, close = authority)]
    pub subject: Account<'info, Subject>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn unappoint_official(ctx: Context<UnappointOfficial>) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let subject_key = ctx.accounts.subject.key();

    kingdom.officials_appointed.retain(|&x| x != subject_key);

    msg!("Official unappointed and removed from the kingdom");

    Ok(())
}

#[derive(Accounts)]
pub struct AddMember<'info> {
    #[account(mut)]
    pub kingdom: Account<'info, Kingdom>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = KingdomMemberRecord::SPACE,
        seeds = [
            b"kingdom_member",
            kingdom.key().as_ref(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub member_record: Account<'info, KingdomMemberRecord>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = kingdom_mint,
        associated_token::authority = authority
    )]
    pub member_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub kingdom_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct KingdomMemberRecord {
    pub kingdom: Pubkey,
    pub member: Pubkey,
    pub joined_at: i64,
    pub role: String,
    pub loyalty: u8,
    pub is_convicted: bool,
    pub bump: u8,
}

impl KingdomMemberRecord {
    pub const SPACE: usize = 8 + // discriminator
        32 + // kingdom
        32 + // member
        8 +  // joined_at
        4 + 32 + // role (String)
        1 +  // loyalty
        1 +  // is_convicted
        1;  // bump
}

pub fn add_member(ctx: Context<AddMember>) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    let member_record = &mut ctx.accounts.member_record;
    let member_token_account = &ctx.accounts.member_token_account;

    // Check if the member has the required tokens
    require!(
        member_token_account.amount >= kingdom.membership_tokens_threshold,
        AbsoluteMonarchyError::InsufficientTokens
    );

    member_record.kingdom = kingdom.key();
    member_record.member = ctx.accounts.authority.key();
    member_record.role = "Citizen".to_string();
    member_record.loyalty = 50; // Start with neutral loyalty
    member_record.is_convicted = false;
    member_record.joined_at = Clock::get()?.unix_timestamp;
    member_record.bump = ctx.bumps.member_record;

    kingdom.total_subjects += 1;

    // Check if the kingdom owns the mint authority
    if kingdom.key() != ctx.accounts.kingdom_mint.mint_authority.unwrap() {
        msg!("Warning: The kingdom does not own the mint authority for this token. Anyone who owns the token can join as a member.");
    }

    emit!(MemberAdded {
        kingdom: kingdom.key(),
        member: ctx.accounts.authority.key(),
        joined_at: member_record.joined_at,
    });

    Ok(())
}

#[event]
pub struct MemberAdded {
    pub kingdom: Pubkey,
    pub member: Pubkey,
    pub joined_at: i64,
}