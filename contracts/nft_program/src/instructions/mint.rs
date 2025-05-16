use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::{
    instruction::{create_metadata_accounts_v3},
    state::Creator,
};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 8
    )]
    pub nft: Account<'info, PrizePalNFT>,
    #[account(mut)]
    pub nft_state: Account<'info, NFTState>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = mpl_token_metadata::id())]
    pub token_metadata_program: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<MintNFT>, rarity: Rarity) -> Result<()> {
    let nft_state = &mut ctx.accounts.nft_state;
    let nft = &mut ctx.accounts.nft;

    // Check minting limits
    require!(nft_state.total_minted < 100_000, ErrorCode::MintLimitReached);
    let rarity_limit = match rarity {
        Rarity::Common => 70_000,
        Rarity::Rare => 20_000,
        Rarity::Epic => 9_000,
        Rarity::Legendary => 1_000,
    };
    require!(
        nft_state.total_minted < rarity_limit,
        ErrorCode::RarityLimitReached
    );

    // Mint NFT
    let cpi_accounts = anchor_spl::token::MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    anchor_spl::token::mint_to(
        CpiContext::new(cpi_program, cpi_accounts),
        1 // Non-fungible (1 token)
    )?;

    // Create metadata (Metaplex)
    let metadata_instruction = create_metadata_accounts_v3(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.metadata_account.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        format!("PrizePal #{}", nft_state.total_minted + 1),
        "PRZPAL".to_string(),
        match rarity {
            Rarity::Common => "https://your-metadata-uri/common.json",
            Rarity::Rare => "https://your-metadata-uri/rare.json",
            Rarity::Epic => "https://your-metadata-uri/epic.json",
            Rarity::Legendary => "https://your-metadata-uri/legendary.json",
        }.to_string(),
        Some(vec![Creator {
            address: ctx.accounts.authority.key(),
            verified: true,
            share: 100,
        }]),
        500, // 5% royalty
        true,
        true,
        None,
        None,
        None,
    );
    anchor_lang::solana_program::program::invoke(
        &metadata_instruction,
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
        ],
    )?;

    // Update state
    nft.owner = ctx.accounts.authority.key();
    nft.rarity = rarity;
    nft.minted_at = Clock::get()?.unix_timestamp;
    nft_state.total_minted += 1;

    Ok(())
}
