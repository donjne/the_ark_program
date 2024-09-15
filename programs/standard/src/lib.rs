use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

declare_id!("7aQvq1fEiDXqK36H7mW8MSTGdnHn6XAHDd9pauZwZXGQ");

#[program]
pub mod standard {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn init_liquidity_pool(
        ctx: Context<InitializePool>,
        initial_liquidity_amount: u64,
        swap_fee: u64,
    ) -> Result<()> {
        initialize_pool(ctx, initial_liquidity_amount, swap_fee)
    }

    pub fn swap_tokens_in_pool(
        ctx: Context<Swap>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        swap(ctx, amount_in, minimum_amount_out)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
