use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use nft_program::instructions::mint::MintNFT;

use crate::state::GameState;
use crate::errors::*;
use nft_program::state::Rarity;

#[derive(Accounts)]
pub struct PurchaseEliteNFT<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub seek_rewards_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    #[account(mut)]
    pub nft_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = nft_program::id())]
    pub nft_program: Program<'info, nft_program::NFTProgram>,
}

pub fn handler(ctx: Context<PurchaseEliteNFT>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;

    // Verify Year 8+ (2032)
    require!(game_state.year >= 8, ErrorCode::MicrotransactionsNotAvailable);

    // Charge $SEEK (mock $50 price)
    let seek_price_usd = 0.01; // Mock; use oracle
    let elite_nft_price = (50.0 / seek_price_usd * 1_000_000_000.0) as u64; // $50 = 5,000 $SEEK
    require!(
        ctx.accounts.player_token_account.amount >= elite_nft_price,
        ErrorCode::InsufficientSeekBalance
    );

    // Transfer $SEEK to rewards pool
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.player_token_account.to_account_info(),
                to: ctx.accounts.seek_rewards_pool.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            }
        ),
        elite_nft_price
    )?;

    // Mint Elite NFT (Epic rarity, simplified)
    nft_program::cpi::mint_nft(
        CpiContext::new(
            ctx.accounts.nft_program.to_account_info(),
            MintNFT {
                nft: ctx.accounts.nft.to_account_info(),
                nft_state: ctx.accounts.nft_state.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                token_account: ctx.accounts.nft_token_account.to_account_info(),
                metadata_account: ctx.accounts.metadata_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
            }
        ),
        Rarity::Epic // Elite NFTs assumed to be Epic
    )?;

    emit!(EliteNFTPurchaseEvent {
        player: ctx.accounts.player.pubkey,
        nft: ctx.accounts.nft_mint.key(),
    });

    Ok(())
}

#[event]
pub struct EliteNFTPurchaseEvent {
    pub player: Pubkey,
    pub nft: Pubkey,
}
