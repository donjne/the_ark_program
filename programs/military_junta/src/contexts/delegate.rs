use anchor_lang::prelude::*;

use anchor_spl::token::{self, TokenAccount, Token, Approve, Mint};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::citizen::Citizen;

#[derive(Accounts)]
pub struct ApproveDelegate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>, 
    #[account(mut)]
    pub citizen: Account<'info, Citizen>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = citizen,
        associated_token::token_program = token_program,
    )]
    pub citizen_ata: Account<'info, TokenAccount>,  
    #[account(signer)]
    /// CHECK: We are passing in this account ourselves
    pub leader: AccountInfo<'info>,  
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn approve_delegate(ctx: Context<ApproveDelegate>, amount: u64) -> Result<()> {
    let cpi_accounts = Approve {
        to: ctx.accounts.citizen_ata.to_account_info(),
        delegate: ctx.accounts.leader.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::approve(cpi_ctx, amount)?;

    Ok(())
}
