use anchor_lang::prelude::*;
use crate::states::{junta::Junta, citizen::Citizen};
use anchor_spl::token::{self, Burn, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ExileCitizen<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub mint: Account<'info, Mint>, 
    #[account(mut)]
    pub leader: Signer<'info>,
    #[account(mut)]
    pub citizen: Account<'info, Citizen>,
    #[account(
        init_if_needed,
        payer = leader,
        associated_token::mint = mint,
        associated_token::authority = citizen,
        associated_token::token_program = token_program,
    )]
    pub citizen_ata: Account<'info, TokenAccount>,  
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn burn_governance_tokens<'info>(
    ctx: Context<ExileCitizen>,
    amount: u64,
) -> Result<()> {

    let junta = &ctx.accounts.junta;

    if junta.leader != ctx.accounts.leader.key() {
        return Err(ErrorCode::Unauthorized.into());
    }

    let cpi_accounts = Burn {
        mint: ctx.accounts.mint.to_account_info(),
        from: ctx.accounts.citizen_ata.to_account_info(),
        authority: ctx.accounts.leader.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::burn(cpi_ctx, amount)?;

    Ok(())
}



pub fn exile_citizen(ctx: Context<ExileCitizen>, target: Pubkey, amount: u64) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let citizen = &mut ctx.accounts.citizen;

   
    require!(junta.leader == ctx.accounts.leader.key(), ErrorCode::Unauthorized);

    
    require!(citizen.authority == target, ErrorCode::InvalidTarget);

    
    citizen.loyalty_score = 0;

    
    citizen.resources = 0;

    burn_governance_tokens(
        ctx,
        amount,
    )?;

    Ok(())
}

// Or rather use the burn constraint to burn tokens
