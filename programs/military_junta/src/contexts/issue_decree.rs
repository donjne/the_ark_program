use anchor_lang::prelude::*;
use crate::states::{junta::Junta, decree::Decree};
use crate::errors::ErrorCode;


#[derive(Accounts)]
#[instruction(content: String)]
pub struct IssueDecree<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(
        init,
        payer = issuer,
        space = 8 + 32 + Decree::MAX_CONTENT_LENGTH + 8
    )]
    pub decree: Account<'info, Decree>,
    #[account(mut)]
    pub issuer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn issue_decree(ctx: Context<IssueDecree>, content: String) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let decree = &mut ctx.accounts.decree;

    require!(
        junta.leader == ctx.accounts.issuer.key() || junta.officers.contains(&ctx.accounts.issuer.key()),
        ErrorCode::Unauthorized
    );
    require!(junta.decrees.len() < Junta::MAX_DECREES, ErrorCode::TooManyDecrees);

    decree.issuer = ctx.accounts.issuer.key();
    decree.content = content;
    decree.issued_at = Clock::get()?.unix_timestamp;

    junta.decrees.push(decree.key());

    Ok(())
}