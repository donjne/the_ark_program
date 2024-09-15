use anchor_lang::prelude::*;
use crate::states::citizen::Citizen;

#[derive(Accounts)]
pub struct InitializeCitizen<'info> {
    #[account(init, payer = authority, space = Citizen::LEN)]
    pub citizen: Account<'info, Citizen>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_citizen(ctx: Context<InitializeCitizen>, authority: Pubkey) -> Result<()> {
    let citizen = &mut ctx.accounts.citizen;

    citizen.authority = authority;
    citizen.loyalty_score = 50; 
    citizen.resources = 0; 
    citizen.is_dissident = false;
    citizen.is_imprisoned = false;
    citizen.imprisonment_end = None;

    Ok(())
}
