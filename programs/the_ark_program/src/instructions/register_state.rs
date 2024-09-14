// To register an instance created from any government state
use anchor_lang::prelude::*;
use crate::state::analytics::ArkAnalytics;

#[derive(Accounts)]
pub struct RegisterGovernment<'info> {
    #[account(mut)]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(init, payer = payer, space = 8 + 32 + 32 + 32 + 32)]
    pub state_info: Account<'info, StateInfo>,
    /// CHECK: This is the program ID of the specific government type
    pub government_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct StateInfo {
    pub name: String,
    pub government_type: GovernmentType,
    pub creator: Pubkey,
    pub created_at: i64,
    pub program_id: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GovernmentType {
    Monarchy,
    Democracy,
    Autocracy,
    Oligarchy,
    Republic,
    Federalism,
    Communism,
    Imperialism,
    Anarchy,
    Colonialism
}

impl Default for GovernmentType {
    fn default() -> Self {
        GovernmentType::Monarchy 
    }
}

#[event]
pub struct StateRegistered {
    pub name: String,
    pub government_type: GovernmentType,
    pub program_id: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum GlobalEvent {
    NaturalDisaster,
    EconomicBoom,
    Pandemic,
    // Add more as needed
}

#[account]
pub struct StateAccount {
    pub name: String,
    pub government_type: GovernmentType,
    pub stability: u8,
    pub at_war: bool,
    // Add other fields as necessary
}
