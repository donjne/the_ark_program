use anchor_lang::prelude::*;
use crate::state::EscrowInfo;

#[derive(Accounts)]
pub struct InitEscrow<'info> {
    #[account(init, payer = signer, space = EscrowInfo::LEN )]
    pub escrow_info: Account<'info, EscrowInfo>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TrackTrade<'info> {
    #[account(mut)]
    pub escrow_info: Account<'info, EscrowInfo>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

impl<'info> TrackTrade<'info> {
    pub fn record_trade(ctx: Context<TrackTrade>, amount: u64) -> Result<()> {
        let escrow_info = &mut ctx.accounts.escrow_info;
        escrow_info.total_trades += 1;
        escrow_info.total_amount_transferred += amount;
        Ok(())
    }

}
