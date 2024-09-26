use anchor_lang::prelude::*;

pub mod state;
pub mod constants;
pub mod instructions;
pub mod utilities;
mod errors;
pub mod interface;

pub use instructions::*;
pub use state::*;
pub use utilities::*;
pub use interface::*;

declare_id!("48qaGS4sA7bqiXYE6SyzaFiAb7QNit1A7vdib7LXhW2V");

#[program]
pub mod the_ark_program {
    use super::*;
    use crate::errors::ErrorCode;



    pub fn initialize_ark(ctx: Context<InitializeArk>) -> Result<()> {
        let analytics = &mut ctx.accounts.ark_analytics;
        analytics.created_at = Clock::get()?.unix_timestamp;
        analytics.governments = Vec::new();
        analytics.total_governments = 0;
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

    pub fn register_government(ctx: Context<RegisterGovernment>, name: String, government_type: GovernmentType) -> Result<()> {
        require!(name.len() <= StateInfo::MAX_NAME_LENGTH, ErrorCode::NameTooLong);

        // pub name: String,
        // pub government_type: GovernmentType,
        // pub creator: Pubkey,
        // pub created_at: i64,
        // pub program_id: Pubkey,
        
        let state_info = &mut ctx.accounts.state_info;

            state_info.name = name;
            state_info.government_type = government_type;
            state_info.creator = ctx.accounts.payer.key();
            state_info.program_id = ctx.accounts.government_program.key();


        // Update ArkAnalytics if needed
        let ark_analytics = &mut ctx.accounts.ark_analytics;
        ark_analytics.total_governments += 1;
        
        emit!(StateRegistered {
            name: state_info.name.clone(),
            government_type: state_info.government_type.clone(),
            program_id: state_info.program_id,
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

    pub fn create_government_treasury(ctx: Context<CreateTreasury>, name: String, authority: Pubkey) -> Result<()> {
        create_treasury(ctx, name, authority)
    }

    pub fn add_new_token_to_treasury(ctx: Context<AddTokenToTreasury>) -> Result<()> {
        add_token_to_treasury(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

