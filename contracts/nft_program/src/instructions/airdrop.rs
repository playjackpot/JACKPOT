use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct AirdropNFT<'info> {
    #[account(mut)]
    pub nft_state: Account<'info, NFTState>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 8
    )]
    pub nft: Account<'info, PrizePalNFT>,
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

pub fn handler(ctx: Context<AirdropNFT>, rarity: Rarity) -> Result<()> {
    let nft_state = &mut ctx.accounts.nft_state;

    // Check airdrop limit
    require!(nft_state.airdropped < 5_000, ErrorCode::AirdropLimitReached);

    // Mint NFT (reuse mint.rs logic)
    let cpi_accounts = anchor_spl::token::MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    anchor_spl::token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1
    )?;

    // Update metadata (simplified, reuse mint.rs logic)
    // Add Metaplex metadata creation here

    // Update state
    let nft = &mut ctx.accounts.nft;
    nft.owner = ctx.accounts.authority.key();
    nft.rarity = rarity;
    nft.minted_at = Clock::get()?.unix_timestamp;
    nft_state.total_minted += 1;
    nft_state.airdropped += 1;

    Ok(())
}
