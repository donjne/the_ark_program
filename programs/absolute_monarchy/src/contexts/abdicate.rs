use anchor_lang::prelude::*;
use crate::states::Monarch;
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct Abdicate<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This account will be the new monarch
    pub heir: UncheckedAccount<'info>,
}

pub fn abdicate(ctx: Context<Abdicate>, heir_name: String) -> Result<()> {
    let monarch = &mut ctx.accounts.monarch;
    monarch.authority = *ctx.accounts.heir.key;
    monarch.name = heir_name;
    monarch.coronation_date = Clock::get()?.unix_timestamp;
    // Reset some stats for the new monarch
    monarch.decrees_issued = 0;
    monarch.wars_declared = 0;
    monarch.royal_judgments = 0;
    monarch.economic_policies_set = 0;

    Ok(())
}