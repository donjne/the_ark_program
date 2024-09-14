use anchor_lang::prelude::*;
use crate::state::escrow::EscrowInfo;
use anchor_spl::token::Token;

#[derive(Accounts)]
pub struct RegisterTrade<'info> {
    #[account(mut)]
    pub escrow_info: Account<'info, EscrowInfo>,
    #[account(mut)]
    pub party_a: Signer<'info>,
    #[account(mut)]
    pub party_b: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RegisterService<'info> {
    #[account(mut)]
    pub escrow_info: Account<'info, EscrowInfo>,
    #[account(mut)]
    pub service_provider: Signer<'info>,
    pub token_program: Program<'info, Token>,
}



