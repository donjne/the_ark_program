use anchor_lang::prelude::*;
use crate::{states::junta::Junta, InitializeJuntaArgs};

#[derive(Accounts)]
#[instruction(args: InitializeJuntaArgs)]
pub struct InitializeJunta<'info> {
    #[account(
        init,
        payer = leader,
        space = 8 + Junta::MAX_NAME_LENGTH + 32 + (32 * Junta::MAX_OFFICERS) + 8 + (32 * Junta::MAX_DECREES) + 1,
        seeds = [Junta::PREFIX_SEED, args.name.as_bytes()], 
        bump
    )]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub leader: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_junta(ctx: Context<InitializeJunta>, args: InitializeJuntaArgs) -> Result<()> {

    // pub name: String,
    // pub leader: Pubkey,
    // pub officers: Vec<Pubkey>,
    // pub resources: u64,
    // pub decrees: Vec<Pubkey>,
    // pub dissent_level: u8,
    // pub control_level: u8,
    // pub governance_token_mint: Option<Pubkey>,
    // pub martial_law_active: bool,
    // pub is_overthrown: bool,
    // pub symbol: String,
    // pub supply: u32,
    // pub minteds: u32,
    // pub price: u64,
    // pub bump: u8,

    let junta = &mut ctx.accounts.junta;

    junta.name = args.name;
    junta.leader = ctx.accounts.leader.key();
    junta.officers = vec![];
    junta.resources = 100;
    junta.decrees = vec![];
    junta.dissent_level = 0;
    junta.supply = args.supply;
    junta.bump = ctx.bumps.junta;
    junta.minteds = 0;
    junta.is_overthrown = false;
    junta.martial_law_active = false;
    junta.collection_price = args.collection_price;
    junta.support_threshold = args.support_threshold;
    Ok(())
}