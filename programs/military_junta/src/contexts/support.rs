use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::states::{junta::Junta, citizen::Citizen, supporters::*};

// const SUPPORT_THRESHOLD: u64 = 1000; // Minimum token amount to be considered a supporter
const SUPPORTERS_PER_LEVEL: usize = 10; // Number of supporters needed to increase control level

#[derive(Accounts)]
pub struct GainSupport<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,

    #[account(
        mut,
        seeds = [b"supporters", junta.key().as_ref()],
        bump
    )]
    pub supporters: Account<'info, Supporters>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub citizen: Account<'info, Citizen>,

    #[account(
        mut,
        constraint = citizen_token_account.owner == citizen.key(),
    )]
    pub citizen_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = junta_token_account.owner == junta.key(),
    )]
    pub junta_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn gain_support(ctx: Context<GainSupport>, amount: u64) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let supporters = &mut ctx.accounts.supporters;

    // Transfer tokens from citizen to junta
    let cpi_accounts = Transfer {
        from: ctx.accounts.citizen_token_account.to_account_info(),
        to: ctx.accounts.junta_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Check if the citizen has transferred enough to be considered a supporter
    let citizen_total_support = supporters.get_support_amount(&ctx.accounts.citizen.key()) + amount;
    
    if citizen_total_support >= junta.support_threshold && !supporters.is_supporter(&ctx.accounts.citizen.key()) {
        supporters.add_supporter(ctx.accounts.citizen.key())?;
        
        // Calculate how many levels to increase
        let new_levels = (supporters.count as usize / SUPPORTERS_PER_LEVEL) - ((supporters.count - 1) as usize / SUPPORTERS_PER_LEVEL);
        
        // Increase the control level of the junta
        junta.control_level = junta.control_level.saturating_add(new_levels as u8);

        // Emit event for new supporter
        emit!(NewSupporterEvent {
            junta: junta.key(),
            supporter: ctx.accounts.citizen.key(),
            amount: citizen_total_support,
        });
    }

    // Update the citizen's total support amount
    supporters.update_support_amount(ctx.accounts.citizen.key(), citizen_total_support)?;

    // Emit event for support contribution
    emit!(SupportContributionEvent {
        junta: junta.key(),
        supporter: ctx.accounts.citizen.key(),
        amount,
        total_support: citizen_total_support,
    });

    Ok(())
}

#[event]
pub struct NewSupporterEvent {
    pub junta: Pubkey,
    pub supporter: Pubkey,
    pub amount: u64,
}

#[event]
pub struct SupportContributionEvent {
    pub junta: Pubkey,
    pub supporter: Pubkey,
    pub amount: u64,
    pub total_support: u64,
}
