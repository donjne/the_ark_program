use anchor_lang::prelude::*;
use crate::states::{Monarch, Subject, Noble};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct GrantNobility<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(mut)]
    pub subject: Account<'info, Subject>,

    #[account(
        init,
        payer = authority,
        space = Noble::space()
    )]
    pub noble: Account<'info, Noble>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn grant_nobility(ctx: Context<GrantNobility>, title: String) -> Result<()> {
    let subject_account = &mut ctx.accounts.subject;
    let noble = &mut ctx.accounts.noble;

    noble.subject = noble.key();
    noble.title = title;
    noble.granted_at = Clock::get()?.unix_timestamp;

    subject_account.nobility_title = Some(noble.title.clone());
    subject_account.loyalty = subject_account.loyalty.saturating_add(10); // Increase loyalty due to ennoblement
    
    msg!("Nobility granted to {}: {}", noble.subject, noble.title);
    Ok(())
}