use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::states::escrow::*;
use crate::errors::RouterError;

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(init, payer = sender, space = 8 + 32 + 32 + 32 + 8 + 200 + 1 + 8)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub recipient: UncheckedAccount<'info>,
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = sender,
        token::mint = mint,
        token::authority = escrow
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct FulfillCondition<'info> {
    #[account(mut, has_one = recipient)]
    pub escrow: Account<'info, Escrow>,
    pub recipient: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReleasePayment<'info> {
    #[account(mut, has_one = recipient, close = sender)]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sender: UncheckedAccount<'info>,
    #[account(mut)]
    pub recipient: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut, has_one = sender, close = sender)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn create_escrow(
    ctx: Context<CreateEscrow>,
    amount: u64,
    condition: String,
    expiry_time: i64,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.sender = ctx.accounts.sender.key();
    escrow.recipient = ctx.accounts.recipient.key();
    escrow.mint = ctx.accounts.mint.key();
    escrow.amount = amount;
    escrow.condition = condition;
    escrow.is_fulfilled = false;
    escrow.expiry_time = expiry_time;

    // Transfer tokens from sender to escrow account
    let cpi_accounts = Transfer {
        from: ctx.accounts.sender_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.sender.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

pub fn fulfill_condition(ctx: Context<FulfillCondition>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    require!(!escrow.is_fulfilled, RouterError::AlreadyFulfilled);
    require!(
        Clock::get()?.unix_timestamp <= escrow.expiry_time,
        RouterError::Expired
    );

    escrow.is_fulfilled = true;
    Ok(())
}

pub fn release_payment(ctx: Context<ReleasePayment>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;
    require!(escrow.is_fulfilled, RouterError::ConditionNotFulfilled);
    require!(
        Clock::get()?.unix_timestamp <= escrow.expiry_time,
        RouterError::Expired
    );

    // Transfer tokens from escrow to recipient
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, escrow.amount)?;

    Ok(())
}

pub fn refund(ctx: Context<Refund>) -> Result<()> {
    let escrow = &ctx.accounts.escrow;
    require!(!escrow.is_fulfilled, RouterError::AlreadyFulfilled);
    require!(
        Clock::get()?.unix_timestamp > escrow.expiry_time,
        RouterError::NotExpired
    );

    // Transfer tokens from escrow back to sender
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.sender_token_account.to_account_info(),
        authority: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, escrow.amount)?;

    Ok(())
}