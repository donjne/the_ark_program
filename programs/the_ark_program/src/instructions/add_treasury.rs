use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::Treasury;

#[derive(Accounts)]
#[instruction(name: String, authority: Pubkey)]
pub struct CreateTreasury<'info> {
    #[account(
        init,
        payer = owner,
        space = Treasury::LEN,
        seeds = [b"treasury", owner.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTokenToTreasury<'info> {
    #[account(mut, has_one = owner)]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = owner,
        token::mint = mint,
        token::authority = treasury,
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

    pub fn create_treasury(ctx: Context<CreateTreasury>, name: String, authority: Pubkey) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.name = name;
        treasury.authority = authority;
        treasury.owner = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn add_token_to_treasury(ctx: Context<AddTokenToTreasury>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        let token_account = &ctx.accounts.token_account;
        treasury.add_token_account(token_account.mint, token_account.key())
    }