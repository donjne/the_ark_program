use anchor_lang::prelude::*;
use crate::states::{junta::Junta, citizen::Citizen};
use crate::errors::ErrorCode;
use anchor_spl::token::{self, Transfer, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;


#[derive(Accounts)]
pub struct ImprisonCitizen<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub citizen: Account<'info, Citizen>,
    #[account(mut)]
    pub leader: Signer<'info>, 
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub citizen_ata: Account<'info, TokenAccount>, // Citizen's governance token account
    #[account(
        init,
        payer = leader,
        associated_token::mint = mint,
        associated_token::authority = junta,
        associated_token::token_program = token_program,
    )]
    pub junta_ata: Account<'info, TokenAccount>, 
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn seize_governance_tokens<'info>(
    ctx: Context<ImprisonCitizen>,
    amount: u64,
) -> Result<()> {

    let junta = &ctx.accounts.junta;

    if junta.leader != ctx.accounts.leader.key() {
        return Err(ErrorCode::Unauthorized.into());
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.citizen_ata.to_account_info(),
        to: ctx.accounts.junta_ata.to_account_info(),
        authority: ctx.accounts.leader.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}


pub fn imprison_citizen(
    ctx: Context<ImprisonCitizen>,
    target: Pubkey,
    end_date: Option<i64>,
    amount: u64,
) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let citizen = &mut ctx.accounts.citizen;
    let leader = &ctx.accounts.leader;

    require!(junta.leader == leader.key(), ErrorCode::Unauthorized);

    require!(citizen.authority == target, ErrorCode::InvalidTarget);

    citizen.loyalty_score = citizen.loyalty_score.saturating_sub(10);

    citizen.is_imprisoned = true;
    citizen.imprisonment_end = end_date;

    seize_governance_tokens(
        ctx,
        amount
    )?;

    Ok(())
}


