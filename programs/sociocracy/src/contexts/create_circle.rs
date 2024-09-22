use anchor_lang::prelude::*;
use crate::states::circle::Circle;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateCircle<'info> {
    #[account(
        init,
        payer = payer,
        space = Circle::space(),
        seeds = [b"circle", name.as_bytes()],
        bump
    )]
    pub circle: Account<'info, Circle>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_circle(ctx: Context<CreateCircle>, name: String) -> Result<()> {
    let circle = &mut ctx.accounts.circle;
    circle.name = name;
    circle.parent_circle = None;
    circle.members = vec![];
    circle.proposals = vec![];
    Ok(())
}