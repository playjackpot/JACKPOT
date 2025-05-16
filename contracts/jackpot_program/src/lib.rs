use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("YourProgramIDHere");

#[program]
pub mod jackpot_program {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        instructions::init_game::handler(ctx)
    }

    pub fn create_hide(ctx: Context<CreateHide>, coordinates: (f64, f64)) -> Result<()> {
        instructions::hide::handler(ctx, coordinates)
    }

    pub fn seek(ctx: Context<Seek>, coordinates: (f64, f64), use_hint: bool) -> Result<()> {
        instructions::seek::handler(ctx, coordinates, use_hint)
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        instructions::burn::handler(ctx, amount)
    }

    pub fn buyback(ctx: Context<Buyback>) -> Result<()> {
        instructions::buyback::handler(ctx)
    }

    pub fn update_rank(ctx: Context<UpdateRank>) -> Result<()> {
        instructions::update_rank::handler(ctx)
    }

    pub fn airdrop_seek(ctx: Context<AirdropSeek>, amount: u64) -> Result<()> {
        instructions::airdrop_seek::handler(ctx, amount)
    }

    pub fn purchase_elite_nft(ctx: Context<PurchaseEliteNFT>) -> Result<()> {
        instructions::microtransactions::handler(ctx)
    }

    pub fn fetch_price(ctx: Context<FetchPrice>) -> Result<()> {
        instructions::fetch_price::handler(ctx)
    }
    pub fn community_event(ctx: Context<CommunityEvent>, reward: EventReward) -> Result<()> {
    instructions::community_events::handler(ctx, reward)
}
