use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("YourProgramIDHere"); // Replace with actual program ID after deployment

#[program]
pub mod jackpot_program {
    use super::*;

    pub fn init_game(ctx: Context<InitGame>) -> Result<()> {
        instructions::init_game::handler(ctx)
    }

    pub fn create_hide(ctx: Context<CreateHide>, coordinates: (f64, f64)) -> Result<()> {
        instructions::hide::handler(ctx, coordinates)
    }

    pub fn seek(ctx: Context<Seek>, coordinates: (f64, f64)) -> Result<()> {
        instructions::seek::handler(ctx, coordinates)
    }

    pub fn burn_seek(ctx: Context<BurnSeek>, amount: u64) -> Result<()> {
        instructions::burn::handler(ctx, amount)
    }

    pub fn buyback_seek(ctx: Context<BuybackSeek>) -> Result<()> {
        instructions::buyback::handler(ctx)

}
    pub fn update_rank(ctx: Context<UpdateRank>) -> Result<()> {
    instructions::update_rank::handler(ctx)
}
    }
}
pub fn purchase_elite_nft(ctx: Context<PurchaseEliteNFT>) -> Result<()> {
    instructions::microtransactions::handler(ctx)
}
