use anchor_lang::prelude::*;
use crate::states::{proposal::{Proposal, ProposalStatus}, vote::StakeAccount, governance::Governance};
use anchor_spl::token::{TokenAccount, Token, Transfer, Mint, transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::states::helpers::*;
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct StakeOnProposal<'info> {
    #[account(mut)]
    pub governance: Box<Account<'info, Governance>>,
    #[account(mut)]
    pub proposal: Box<Account<'info, Proposal>>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"stake", proposal.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake: Box<Account<'info, StakeAccount>>,
    #[account(
        mut,
        constraint = mint.key() == governance.spl_mint @ ErrorCode::InvalidMint
    )]
    pub mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = stake,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = governance,
        associated_token::token_program = token_program,
    )]
    pub proposal_account: Box<Account<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn stake(ctx: Context<StakeOnProposal>, amount: u64, lock_period: u8) -> Result<()> {
    let governance = &ctx.accounts.governance;
    let proposal = &mut ctx.accounts.proposal;
    let stake_account = &mut ctx.accounts.stake;
    let clock = Clock::get()?;

    require!(
        ctx.accounts.mint.key() == governance.spl_mint,
        ErrorCode::InvalidMint
    );

    require!(
        amount >= governance.min_stake_amount,
        ErrorCode::StakeTooLow
    );

    require!(proposal.status == ProposalStatus::Active, ErrorCode::ProposalNotActive);

    // Transfer tokens from user to proposal account
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.proposal_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    stake_account.user = ctx.accounts.user.key();
    stake_account.amount += amount;
    stake_account.lock_end = clock.unix_timestamp + calculate_lock_duration(lock_period);
    stake_account.conviction_multiplier = calculate_conviction_multiplier(lock_period);

    Ok(())
}