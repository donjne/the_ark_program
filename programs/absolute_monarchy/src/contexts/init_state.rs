use anchor_lang::prelude::*;
use crate::states::{Monarch, Treasury, Kingdom};

#[derive(Accounts)]
pub struct InitializeKingdom<'info> {
    #[account(init, payer = authority, space = Kingdom::space())]
    pub kingdom: Account<'info, Kingdom>,

    #[account(init, payer = authority, space = Monarch::space())]
    pub monarch: Account<'info, Monarch>,

    #[account(init, payer = authority, space = Treasury::space())]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_kingdom(
    ctx: Context<InitializeKingdom>,
    kingdom_name: String,
    monarch_name: String,
    divine_mandate: String
) -> Result<()> {
    let kingdom = &mut ctx.accounts.kingdom;
    kingdom.name = kingdom_name;
    kingdom.monarch = ctx.accounts.monarch.key();
    kingdom.established_at = Clock::get()?.unix_timestamp;

    let monarch = &mut ctx.accounts.monarch;
    monarch.authority = *ctx.accounts.authority.key;
    monarch.name = monarch_name;
    monarch.divine_mandate = divine_mandate;
    monarch.coronation_date = Clock::get()?.unix_timestamp;
    monarch.decrees_issued = 0;
    monarch.wars_declared = 0;
    monarch.royal_judgments = 0;
    monarch.economic_policies_set = 0;
    monarch.pardons_granted = 0;
    monarch.policies_implemented = 0;

    let treasury = &mut ctx.accounts.treasury;
    treasury.balance = 0;
    treasury.taxes_collected = 0;
    treasury.last_collection_date = Clock::get()?.unix_timestamp;
    treasury.royal_expenses = 0;
    treasury.operational_expenses = 0;
    treasury.military_funding = 0;

    Ok(())
}