use anchor_lang::prelude::*;

declare_id!("ADp9DgS9ZpsVDCXb4ysDjJoB1d8cL3CUmm4ErwVtqWzu");

#[program]
pub mod absolute_monarchy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
