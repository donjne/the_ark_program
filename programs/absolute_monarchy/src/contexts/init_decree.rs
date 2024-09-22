use anchor_lang::prelude::*;
use crate::states::{Monarch, Decree, DecreeType};
use crate::error::AbsoluteMonarchyError;

#[derive(Accounts)]
pub struct DecreeContext<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(
        init,
        payer = authority,
        space = Decree::space()
    )]
    pub decree: Account<'info, Decree>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn decree(ctx: Context<DecreeContext>, decree_text: String, decree_type: DecreeType) -> Result<()> {
    let monarch = &mut ctx.accounts.monarch;
    let decree = &mut ctx.accounts.decree;

    decree.id = monarch.decrees_issued + 1;
    decree.text = decree_text;
    decree.decree_type = decree_type;
    decree.issued_at = Clock::get()?.unix_timestamp;
    decree.is_active = true;

    monarch.decrees_issued += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct RepealDecree<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(mut)]
    pub decree: Account<'info, Decree>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn repeal_decree(ctx: Context<RepealDecree>, decree_id: u64) -> Result<()> {
    let decree = &mut ctx.accounts.decree;
    require!(decree.id == decree_id, AbsoluteMonarchyError::InvalidDecree);
    decree.is_active = false;
    Ok(())
}