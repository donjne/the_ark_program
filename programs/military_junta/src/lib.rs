use anchor_lang::prelude::*;
use the_ark_program::cpi::accounts::RegisterGovernment;
use the_ark_program::program::TheArkProgram;
use the_ark_program::state::analytics::ArkAnalytics;
use the_ark_program::cpi::register_government;
use the_ark_program::instructions::register_state::StateInfo;
use the_ark_program::instructions::register_state::GovernmentType;

pub mod states;
pub mod contexts;
mod errors;

pub use contexts::*;
pub use states::*;

declare_id!("2fPj7RDkm4FJouSo6DE6vHbE5rjTvdZPnnxJUgFvYVm2");

#[program]
pub mod military_junta {
    use super::*;

    pub fn initialize_and_register_government(ctx: Context<Initialize>) -> Result<()> {
        let state_info = &mut ctx.accounts.state_info;
        state_info.name = "Military Junta".to_string(); 
        state_info.government_type = GovernmentType::Autocracy; 
        state_info.creator = ctx.accounts.creator.key();
        state_info.created_at = Clock::get()?.unix_timestamp;
        state_info.program_id = ctx.accounts.government_program.key(); 

        // Register the government with Ark using CPI
        let cpi_program = ctx.accounts.ark_program.to_account_info();
        let cpi_accounts = RegisterGovernment {
            ark_analytics: ctx.accounts.ark_analytics.to_account_info(),
            state_info: ctx.accounts.state_info.to_account_info(),
            government_program: ctx.accounts.government_program.to_account_info(),
            payer: ctx.accounts.creator.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        register_government(cpi_ctx, ctx.accounts.state_info.key())?;

        Ok(())
    }

    pub fn initialize_mil_junta(ctx: Context<InitializeJunta>, args: InitializeJuntaArgs) -> Result<()> {
        initialize_junta(ctx, args)
    }

    pub fn appoint_officers(ctx: Context<AppointOfficer>, rank: u8) -> Result<()> {
        appoint_officer(ctx, rank)
    }

    pub fn issue_decrees(ctx: Context<IssueDecree>, content: String) -> Result<()> {
        issue_decree(ctx, content)
    }

    pub fn suppress_dissents(ctx: Context<SuppressDissent>, target: Pubkey) -> Result<()> {
        suppress_dissent(ctx, target)
    }

    pub fn manage_resources(ctx: Context<ManageResources>, action: ResourceAction, amount: u64) -> Result<()> {
        resources(ctx, action, amount)
    }

    pub fn delegate_token_account(ctx: Context<ApproveDelegate>, amount: u64) -> Result<()> {
        approve_delegate(ctx, amount)
    }

    pub fn exile(ctx: Context<ExileCitizen>, target: Pubkey, amount_to_burn: u64) -> Result<()> {
        exile_citizen(ctx, target, amount_to_burn)
    }

    pub fn imprison(ctx: Context<ImprisonCitizen>, target: Pubkey, end_date: Option<i64>, amount_to_seize: u64) -> Result<()> {
        imprison_citizen(ctx, target, end_date, amount_to_seize)
    }

    pub fn init_citizen(ctx: Context<InitializeCitizen>, authority: Pubkey) -> Result<()> {
        initialize_citizen(ctx, authority)
    }

    pub fn impose_martiallaw(ctx: Context<ImposeMartialLaw>) -> Result<()> {
        martial_law(ctx)
    }

    pub fn start_rebel(ctx: Context<StartRebellion>, rebellion_scale: u64) -> Result<()> {
        start_rebellion(ctx, rebellion_scale)
    }

    pub fn reward_citizen(ctx: Context<RewardLoyalty>, amount: u64) -> Result<()> {
        reward_loyalty(ctx, amount)
    }

    pub fn support_junta(ctx: Context<GainSupport>,  amount: u64) -> Result<()> {
        gain_support(ctx, amount)
    }

    pub fn change_leadership(ctx: Context<TransferLeadership>, new_leader: Pubkey) -> Result<()> {
        transfer_leadership(ctx, new_leader)
    }

    pub fn mint_nfts_to_citizen(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        mint_nft(ctx, args)
    }

    pub fn mint_sbts_to_citizen(ctx: Context<MintJuntaSbt>, args: InitializeSbtArgs) -> Result<()> {
        mint_sbt(ctx, args)
    }

    pub fn mint_spl_tokens(ctx: Context<MintTokens>, amount_to_treasury: u64, amount_to_citizen: u64) -> Result<()> {
        mint_tokens(ctx,  amount_to_treasury, amount_to_citizen)
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub ark_analytics: Account<'info, ArkAnalytics>,
    #[account(init, payer = creator, space = 8 + 32 + 32 + 32 + 32 + 32 + 8)]
    pub state_info: Account<'info, StateInfo>,
    /// CHECK: This is the program ID of the specific government type
    pub government_program: UncheckedAccount<'info>,
    pub ark_program: Program<'info, TheArkProgram>,
    pub system_program: Program<'info, System>,
}
