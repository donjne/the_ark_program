use anchor_lang::prelude::*;
use crate::states::{Monarch, Subject};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct AppointOfficial<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(init, payer = authority, space = Subject::space())]
    pub subject: Account<'info, Subject>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn appoint_official(ctx: Context<AppointOfficial>, role: String, jurisdiction: String) -> Result<()> {
    let subject_account = &mut ctx.accounts.subject;
    subject_account.key = subject_account.key();
    subject_account.role = role;
    subject_account.jurisdiction = jurisdiction;
    subject_account.loyalty = 100; // Start with full loyalty
    subject_account.wealth = 0;
    subject_account.is_convicted = false;

    Ok(())
}