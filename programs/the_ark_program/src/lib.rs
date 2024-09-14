use anchor_lang::prelude::*;

pub mod state;
pub mod constants;
pub mod instructions;
pub mod utilities;

pub use instructions::*;
pub use state::*;
pub use utilities::*;

declare_id!("48qaGS4sA7bqiXYE6SyzaFiAb7QNit1A7vdib7LXhW2V");

#[program]
pub mod the_ark_program {
    use super::*;

    pub fn initialize_ark(ctx: Context<InitializeArk>) -> Result<()> {
        let analytics = &mut ctx.accounts.ark_analytics;
        analytics.created_at = Clock::get()?.unix_timestamp;
        analytics.governments = Vec::new();
        analytics.no_of_governments = 0;
        analytics.polls = 0;
        analytics.approved = 0;
        analytics.rejected = 0;
        analytics.points = 0;
        Ok(())
    }

    pub fn init_escrow(ctx: Context<InitEscrow>) -> Result<()> {
        let escrow_info = &mut ctx.accounts.escrow_info;
        escrow_info.total_trades = 0;
        escrow_info.trades = Vec::new();
        escrow_info.total_services = 0;
        escrow_info.total_fees_collected = 0;
        escrow_info.total_amount_transferred = 0;
        Ok(())
    }

    pub fn register_trades(ctx: Context<RegisterTrade>, trade_address: Pubkey) -> Result<()> {
        let escrow_info = &mut ctx.accounts.escrow_info;
        escrow_info.total_trades += 1;
        escrow_info.trades.push(trade_address);

        Ok(())
    }

    pub fn register_services(ctx: Context<RegisterService>, services_address: Pubkey) -> Result<()> {
        let escrow_info = &mut ctx.accounts.escrow_info;
        escrow_info.total_services += 1;
        escrow_info.services.push(services_address);

        Ok(())
    }

    pub fn register_government(ctx: Context<RegisterGovernment>, government_address: Pubkey) -> Result<()> {
        let analytics = &mut ctx.accounts.ark_analytics;
        analytics.governments.push(government_address);
        analytics.no_of_governments += 1;
        
        emit!(StateRegistered {
            name: ctx.accounts.state_info.name.clone(),
            government_type: ctx.accounts.state_info.government_type.clone(),
            program_id: ctx.accounts.government_program.key(),
        });

        Ok(())
    }

    pub fn update_analytics(ctx: Context<UpdateAnalytics>, approved: bool) -> Result<()> {
        let analytics = &mut ctx.accounts.ark_analytics;
        if approved {
            analytics.approved += 1;
        } else {
            analytics.rejected += 1;
        }
        analytics.polls += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
