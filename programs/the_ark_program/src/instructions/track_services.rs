use anchor_lang::prelude::*;
use crate::state::EscrowInfo;


#[derive(Accounts)]
pub struct TrackService<'info> {
    #[account(mut)]
    pub escrow_info: Account<'info, EscrowInfo>,
    #[account(mut)]
    pub signer: Signer<'info>,
}


impl<'info> TrackService<'info> {
    pub fn record_service(ctx: Context<TrackService>, amount: u64, fee: u64) -> Result<()> {
        let escrow_info = &mut ctx.accounts.escrow_info;
        escrow_info.total_services += 1;
        escrow_info.total_amount_transferred += amount;
        escrow_info.total_fees_collected += fee;
        Ok(())
    }
}