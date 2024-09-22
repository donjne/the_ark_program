use anchor_lang::prelude::*;
use crate::states::{Monarch, EconomicPolicy, Subject, EconomicPolicyType};
use crate::error::AbsoluteMonarchyError;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};


#[derive(Accounts)]
pub struct SetEconomicPolicy<'info> {
    #[account(mut, has_one = authority @ AbsoluteMonarchyError::NotMonarch)]
    pub monarch: Account<'info, Monarch>,

    #[account(
        init,
        payer = authority,
        space = EconomicPolicy::space()
    )]
    pub economic_policy: Account<'info, EconomicPolicy>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn set_economic_policy(
    ctx: Context<SetEconomicPolicy>, 
    policy: EconomicPolicyType, 
    income_tax_rate: u8,
    property_tax_rate: u8,
    trade_tax_rate: u8,
    luxury_tax_rate: u8
) -> Result<()> {
    require!(income_tax_rate <= 100 && property_tax_rate <= 100 && trade_tax_rate <= 100 && luxury_tax_rate <= 100, 
        AbsoluteMonarchyError::InvalidTaxRate);

    let economic_policy = &mut ctx.accounts.economic_policy;
    economic_policy.policy_type = policy.clone();
    economic_policy.implemented_at = Clock::get()?.unix_timestamp;
    economic_policy.income_tax_rate = income_tax_rate;
    economic_policy.property_tax_rate = property_tax_rate;
    economic_policy.trade_tax_rate = trade_tax_rate;
    economic_policy.luxury_tax_rate = luxury_tax_rate;

    ctx.accounts.monarch.economic_policies_set += 1;
    msg!("New economic policy set: {:?} with tax rates: Income {}%, Property {}%, Trade {}%, Luxury {}%", 
        policy, income_tax_rate, property_tax_rate, trade_tax_rate, luxury_tax_rate);
    Ok(())
}

#[derive(Accounts)]
pub struct PayTax<'info> {
    #[account(mut)]
    pub subject: Account<'info, Subject>,

    #[account(mut)]
    pub subject_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub economic_policy: Account<'info, EconomicPolicy>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Debug)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TaxType {
    Income,
    Property,
    Trade,
    Luxury,
}

pub fn pay_tax(ctx: Context<PayTax>, tax_type: TaxType, taxable_amount: u64) -> Result<()> {
    let economic_policy = &ctx.accounts.economic_policy;
    let tax_rate = match tax_type {
        TaxType::Income => economic_policy.income_tax_rate,
        TaxType::Property => economic_policy.property_tax_rate,
        TaxType::Trade => economic_policy.trade_tax_rate,
        TaxType::Luxury => economic_policy.luxury_tax_rate,
    };

    let tax_amount = (taxable_amount as u128 * tax_rate as u128 / 100) as u64;

    // Transfer tax from subject to treasury
    let transfer_instruction = Transfer {
        from: ctx.accounts.subject_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        ),
        tax_amount,
    )?;

    // Update subject's wealth
    let subject = &mut ctx.accounts.subject;
    subject.wealth = subject.wealth.saturating_sub(tax_amount);

    msg!("Tax paid: {} for {:?} tax on amount {}", tax_amount, tax_type, taxable_amount);
    Ok(())
}