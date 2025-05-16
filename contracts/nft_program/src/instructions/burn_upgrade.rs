use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, Mint, Token, TokenAccount};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct BurnUpgrade<'info> {
    #[account(mut)]
    pub nft_state: Account<'info, NFTState>,
    #[account(mut, close = authority)]
    pub old_nft: Account<'info, PrizePalNFT>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1 + 8
    )]
    pub new_nft: Account<'info, PrizePalNFT>,
    #[account(mut)]
    pub old_mint: Account<'info, Mint>,
    #[account(mut)]
    pub old_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub new_mint: Account<'info, Mint>,
    #[account(mut)]
    pub new_token_account: Account<'info, TokenAccount>,
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

pub fn handler(ctx: Context<BurnUpgrade>, target_rarity: Rarity) -> Result<()> {
    let old_nft = &ctx.accounts.old_nft;
    let new_nft = &mut ctx.accounts.new_nft;
    let nft_state = &mut ctx.accounts.nft_state;

    // Validate rarity upgrade
    require!(
        can_upgrade(&old_nft.rarity, &target_rarity),
        ErrorCode::InvalidRarityUpgrade
    );

    // Burn old NFT
    let cpi_accounts = Burn {
        mint: ctx.accounts.old_mint.to_account_info(),
        to: ctx.accounts.old_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    anchor_spl::token::burn(
        CpiContext::new(cpi_program, cpi_accounts),
        1
    )?;

    // Mint new NFT (simplified, reuse mint.rs logic)
    let cpi_accounts = anchor_spl::token::MintTo {
        mint: ctx.accounts.new_mint.to_account_info(),
        to: ctx.accounts.new_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    anchor_spl::token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1
    )?;

    // Update metadata (simplified)
    // Add Metaplex metadata update logic here (similar to mint.rs)

    // Update state
    new_nft.owner = ctx.accounts.authority.key();
    new_nft.rarity = target_rarity;
    new_nft.minted_at = Clock::get()?.unix_timestamp;
    nft_state.total_minted += 1;

    Ok(())
}

fn can_upgrade(current: &Rarity, target: &Rarity) -> bool {
    matches!(
        (current, target),
        (Rarity::Common, Rarity::Rare) |
        (Rarity::Rare, Rarity::Epic) |
        (Rarity::Epic, Rarity::Legendary)
    )
}
