use anchor_lang::prelude::*;
use crate::states::junta::Junta;
use crate::errors::ErrorCode;


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ResourceAction {
    Collect,
    Distribute,
}

#[derive(Accounts)]
pub struct ManageResources<'info> {
    #[account(mut)]
    pub junta: Account<'info, Junta>,
    #[account(mut)]
    pub manager: Signer<'info>,
}

pub fn resources(ctx: Context<ManageResources>, action: ResourceAction, amount: u64) -> Result<()> {
    let junta = &mut ctx.accounts.junta;

    require!(
        junta.leader == ctx.accounts.manager.key() || junta.officers.contains(&ctx.accounts.manager.key()),
        ErrorCode::Unauthorized
    );

    match action {
        ResourceAction::Collect => {
            junta.resources = junta.resources.checked_add(amount).unwrap();
        },
        ResourceAction::Distribute => {
            require!(junta.resources >= amount, ErrorCode::InsufficientResources);
            junta.resources = junta.resources.checked_sub(amount).unwrap();
        },
    }

    Ok(())
}