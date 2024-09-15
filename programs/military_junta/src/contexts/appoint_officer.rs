use anchor_lang::prelude::*;
use crate::states::{junta::Junta, officer::Officer};
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct AppointOfficer<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(
        init,
        payer = leader,
        space = 8 + 32 + 1 + 8
    )]
    pub officer: Account<'info, Officer>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn appoint_officer(ctx: Context<AppointOfficer>, rank: u8) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let officer = &mut ctx.accounts.officer;

    require!(junta.leader == ctx.accounts.leader.key(), ErrorCode::Unauthorized);
    require!(junta.officers.len() < Junta::MAX_OFFICERS, ErrorCode::TooManyOfficers);

    officer.authority = ctx.accounts.leader.key();
    officer.rank = rank;
    officer.appointed_at = Clock::get()?.unix_timestamp;

    junta.officers.push(officer.key());

    Ok(())
}