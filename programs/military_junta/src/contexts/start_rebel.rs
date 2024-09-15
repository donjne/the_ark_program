use anchor_lang::prelude::*;
use crate::states::{junta::Junta, citizen::Citizen, rebel::Rebel};
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct StartRebellion<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,

    #[account(mut)]
    pub rebel_leader: Signer<'info>,  

    #[account(mut)]
    pub rebels: Account<'info, Rebel>,  

    #[account(mut)]
    pub citizen: Account<'info, Citizen>, 
}


pub fn start_rebellion(ctx: Context<StartRebellion>, rebellion_scale: u64) -> Result<()> {
    let junta = &mut ctx.accounts.junta;
    let rebels = &mut ctx.accounts.rebels;
    let citizen = &ctx.accounts.citizen;

    require!(rebels.count as u64 >= rebellion_scale, ErrorCode::NotEnoughRebels);

    if citizen.loyalty_score < 20 {
        let citizen_data = Citizen {
            authority: citizen.authority,
            loyalty_score: citizen.loyalty_score,
            resources: citizen.resources,
            is_dissident: citizen.is_dissident,
            is_imprisoned: citizen.is_imprisoned,
            imprisonment_end: citizen.imprisonment_end,
        };

        rebels.add_rebel(citizen_data)?;

        
        junta.dissent_level = junta.dissent_level.saturating_add(1);
        junta.control_level = junta.control_level.saturating_sub(1);


        let mut updated_citizen = ctx.accounts.citizen.clone();
        updated_citizen.loyalty_score = updated_citizen.loyalty_score.saturating_sub(5);
    }

    if junta.control_level <= 0 {
        junta.is_overthrown = true;
        msg!("The Junta has been overthrown by the rebellion!");
    } else {
        msg!("The rebellion was unsuccessful, but dissent has increased.");
    }

    Ok(())
}

