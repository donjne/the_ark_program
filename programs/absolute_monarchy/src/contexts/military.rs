use anchor_lang::prelude::*;
use crate::states::{Monarch, War, Treasury};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct DeclareWar<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(
        init,
        payer = authority,
        space = War::space()
    )]
    pub war: Account<'info, War>,

    /// CHECK: This is not our account, we just read from it
    pub enemy_program: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn declare_war(ctx: Context<DeclareWar>, reason: String) -> Result<()> {
    let war = &mut ctx.accounts.war;
    war.enemy_program = *ctx.accounts.enemy_program.key;
    war.reason = reason;
    war.declared_at = Clock::get()?.unix_timestamp;
    war.is_active = true;
    war.battles_won = 0;
    war.battles_lost = 0;

    ctx.accounts.monarch.wars_declared += 1;

    // Here you would typically invoke a CPI to notify the enemy program
    // For now, we'll just log the declaration
    msg!("War declared against program {} for reason: {}", war.enemy_program, war.reason);
    Ok(())
}

// #[derive(Accounts)]
// pub struct MobilizeArmy<'info> {
//     #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
//     pub monarch: Account<'info, Monarch>,

//     #[account(mut)]
//     pub treasury: Account<'info, Treasury>,

//     #[account(mut)]
//     pub authority: Signer<'info>,
// }

// pub fn mobilize_army(ctx: Context<MobilizeArmy>, troops: u64, target: Pubkey) -> Result<()> {
//     let treasury = &mut ctx.accounts.treasury;
//     let cost = troops * 100; // Each troop costs 100 units to mobilize
//     require!(treasury.balance >= cost, AbsoluteMonarchyError::InsufficientFunds);

//     treasury.balance -= cost;
//     treasury.military_funding += cost;

//     msg!("Mobilized {} troops against target {}", troops, target);
//     Ok(())
// }
