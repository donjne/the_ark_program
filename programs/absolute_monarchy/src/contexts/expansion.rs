use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use crate::error::AbsoluteMonarchyError;
use crate::{Division, Monarch};

pub const ONE_MONTH_IN_SECONDS: i64 = 86_400 * 30;

#[derive(Accounts)]
pub struct ExpandOrganization<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(mut)]
    pub main_treasury: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = Division::space()
    )]
    pub division: Account<'info, Division>,

    #[account(init, payer = authority, token::mint = mint, token::authority = division)]
    pub division_treasury: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn expand_organization(ctx: Context<ExpandOrganization>, new_division_name: String, budget: u64) -> Result<()> {
    let division = &mut ctx.accounts.division;
    division.name = new_division_name;
    division.manager = *ctx.accounts.authority.key;
    division.established_at = Clock::get()?.unix_timestamp;
    division.last_transfer_at = division.established_at;
    division.treasury = ctx.accounts.division_treasury.key();

    // Transfer tokens from main treasury to division treasury
    let transfer_instruction = Transfer {
        from: ctx.accounts.main_treasury.to_account_info(),
        to: ctx.accounts.division_treasury.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        ),
        budget,
    )?;

    msg!("New division '{}' established with budget {}", division.name, budget);
    Ok(())
}

#[derive(Accounts)]
pub struct TransferRevenue<'info> {
    #[account(mut)]
    pub division: Account<'info, Division>,

    #[account(mut, constraint = division_treasury.key() == division.treasury @ AbsoluteMonarchyError::InvalidTreasury)]
    pub division_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub main_treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    /// CHECK: This is safe because we're using it as a signer only
    pub division_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn transfer_revenue(ctx: Context<TransferRevenue>) -> Result<()> {
    let division = &mut ctx.accounts.division;
    let current_time = Clock::get()?.unix_timestamp;

    // Check if a month has passed since the last transfer
    if current_time - division.last_transfer_at < ONE_MONTH_IN_SECONDS {
        return Err(AbsoluteMonarchyError::TransferTooSoon.into());
    }

    let division_balance = ctx.accounts.division_treasury.amount;
    let transfer_amount = division_balance / 10; // 10% of the balance

    if transfer_amount == 0 {
        return Err(AbsoluteMonarchyError::InsufficientFunds.into());
    }

    // Transfer 10% from division treasury to main treasury
    let transfer_instruction = Transfer {
        from: ctx.accounts.division_treasury.to_account_info(),
        to: ctx.accounts.main_treasury.to_account_info(),
        authority: ctx.accounts.division_authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        ),
        transfer_amount,
    )?;

    division.last_transfer_at = current_time;

    msg!("Transferred {} from division '{}' to main treasury", transfer_amount, division.name);

    // Check if division treasury is now empty
    if ctx.accounts.division_treasury.amount == 0 {
        msg!("Division '{}' treasury is empty. Closing division.", division.name);
        // Close the division account and transfer lamports to the main treasury
        let destination = ctx.accounts.main_treasury.to_account_info();
        let source = ctx.accounts.division.to_account_info();
        let dest_starting_lamports = destination.lamports();
        **destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(source.lamports())
            .unwrap();
        **source.lamports.borrow_mut() = 0;
    }

    Ok(())
}