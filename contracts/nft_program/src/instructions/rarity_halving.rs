use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::{PrizePalNFT, Rarity};
use crate::errors::*;

#[derive(Accounts)]
pub struct RarityHalving<'info> {
    #[account(mut)]
    pub nft: Account<'info, PrizePalNFT>,
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(address = mpl_token_metadata::id())]
    pub token_metadata_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<RarityHalving>) -> Result<()> {
    let nft = &mut ctx.accounts.nft;
    let current_year = Clock::get()?.unix_timestamp / 31_536_000; // Seconds per year (approx)
    let start_year = 2025;

    // Verify year is 2028 or later
    require!(current_year >= start_year + 3, ErrorCode::InvalidRarityHalvingTime);

    // Update rarity
    let new_rarity = match nft.rarity {
        Rarity::Legendary => Rarity::Epic,
        Rarity::Epic => Rarity::Rare,
        Rarity::Rare => Rarity::Common,
        Rarity::Common => Rarity::Common, // No change for Common
    };

    nft.rarity = new_rarity;

    // Update metadata (simplified; update Metaplex metadata URI)
    let metadata_instruction = mpl_token_metadata::instruction::update_metadata_accounts_v2(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.metadata_account.key(),
        ctx.accounts.authority.key(),
        None,
        Some(mpl_token_metadata::state::DataV2 {
            name: format!("PrizePal #{}", ctx.accounts.nft.minted_at),
            symbol: "PRZPAL".to_string(),
            uri: match new_rarity {
                Rarity::Common => "https://your-metadata-uri/common.json",
                Rarity::Rare => "https://your-metadata-uri/rare.json",
                Rarity::Epic => "https://your-metadata-uri/epic.json",
                Rarity::Legendary => "https://your-metadata-uri/legendary.json",
            },
            creators: None,
            seller_fee_basis_points: 500, // 5% royalty
            collection: None,
            uses: None,
        }),
        None,
        None,
    );
    anchor_lang::solana_program::program::invoke(
        &metadata_instruction,
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
        ],
    )?;

    emit!(RarityHalvedEvent {
        nft: ctx.accounts.nft.key(),
        old_rarity: nft.rarity.clone(),
        new_rarity,
    });

    Ok(())
}

#[event]
pub struct RarityHalvedEvent {
    pub nft: Pubkey,
    pub old_rarity: Rarity,
    pub new_rarity: Rarity,
}
