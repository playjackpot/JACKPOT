use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("YourNFTProgramIDHere"); // Replace with actual program ID after deployment

#[program]
pub mod nft_program {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>, rarity: Rarity) -> Result<()> {
        instructions::mint::handler(ctx, rarity)
    }

    pub fn burn_upgrade(ctx: Context<BurnUpgrade>, target_rarity: Rarity) -> Result<()> {
        instructions::burn_upgrade::handler(ctx, target_rarity)
    }

    pub fn airdrop_nft(ctx: Context<AirdropNFT>, rarity: Rarity) -> Result<()> {
        instructions::airdrop::handler(ctx, rarity)
    }
}
