use anchor_lang::prelude::*;

#[account]
pub struct Junta {
    pub name: String,
    pub leader: Pubkey,
    pub officers: Vec<Pubkey>,
    pub resources: u64,
    pub decrees: Vec<Pubkey>,
    pub dissent_level: u8,
    pub control_level: u8,
    pub mint: Option<Pubkey>,
    pub martial_law_active: bool,
    pub support_threshold: u64,
    pub is_overthrown: bool,
    pub symbol: String,
    pub supply: u32,
    pub minteds: u32,
    pub collection_price: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeJuntaArgs {
    pub name: String,
    pub supply: u32,
    pub symbol: String,
    pub support_threshold: u64,
    pub collection_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeSbtArgs {
    pub name: String,
    pub supply: u32,
    pub symbol: String,
    pub transferrable: bool,
    pub uri: String,
}

impl Junta {
    pub const MAX_NAME_LENGTH: usize = 50;
    pub const MAX_OFFICERS: usize = 10;
    pub const MAX_DECREES: usize = 100;
    pub const PREFIX_SEED: &'static [u8] = b"junta";
    pub const SPL_PREFIX_SEED: &'static [u8] = b"spl";
    pub const NFT_PREFIX_SEED: &'static [u8] = b"nft";
    pub const SPACE: usize = 8  // Discriminator
    + 4 + Self::MAX_NAME_LENGTH  // Name (size prefixed string)
    + 32  // Pubkey (leader)
    + 4 + (32 * Self::MAX_OFFICERS)  // Vec<Pubkey> (officers)
    + 8   // u64 (resources)
    + 4 + (32 * Self::MAX_DECREES)  // Vec<Pubkey> (decrees)
    + 1   // u8 (dissent_level)
    + 1   // u8 (control_level)
    + 32  // Pubkey (governance_token_mint)
    + 1   // bool (martial_law_active)
    + 1   // bool (is_overthrown)
    + 4 + 10  // Symbol (size prefixed string)
    + 4   // u32 (supply)
    + 4   // u32 (minteds)
    + 8   // u64 (price)
    + 32  // Pubkey (treasury_account)
    + 1;  // u8 (bump)
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintNftArgs {
    pub name: String,
    pub uri: String,
}