use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use nft_program::state::Rarity;

use crate::state::{GameState, Player};
use crate::errors::*;

#[derive(Accounts)]
pub struct CommunityEvent<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub seek_rewards_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub nft_mint: Option<Account<'info, Mint>>,
    #[account(mut)]
    pub nft_token_account: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metadata_account: Option<UncheckedAccount<'info>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = nft_program::id())]
    pub nft_program: Program<'info, nft_program::NFTProgram>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum EventReward {
    Seek { amount: u64 },
    NFT { rarity: Rarity },
}

pub fn handler(ctx: Context<CommunityEvent>, reward: EventReward) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let player = &ctx.accounts.player;

    // Verify admin
    require!(is_admin(ctx.accounts.authority.key()), ErrorCode::Unauthorized);

    // Check player eligibility (e.g., active in last 30 days)
    require!(
        player.seeks > 0 || player.hides > 0,
        ErrorCode::IneligibleForEvent
    );

    match reward {
        EventReward::Seek { amount } => {
            // Check airdrop limit (15M $SEEK total)
            require!(
                game_state.seek_rewards_pool + amount <= 15_000_000_000_000_000,
                ErrorCode::AirdropLimitReached
            );

            // Transfer $SEEK
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.seek_rewards_pool.to_account_info(),
                        to: ctx.accounts.player_token_account.to_account_info(),
                        authority: ctx.accounts.game_state.to_account_info(),
                    }
                ),
                amount
            )?;

            game_state.seek_rewards_pool = game_state.seek_rewards_pool.saturating_sub(amount);
        }
        EventReward::NFT { rarity } => {
            let nft_mint = ctx.accounts.nft_mint.as_ref().ok_or(ErrorCode::NoNFTProvided)?;
            let nft_token_account = ctx.accounts.nft_token_account.as_ref().ok_or(ErrorCode::NoNFTProvided)?;
            let metadata_account = ctx.accounts.metadata_account.as_ref().ok_or(ErrorCode::NoNFTProvided)?;

            // Mint NFT via nft_program
            nft_program::cpi::mint_nft(
                CpiContext::new(
                    ctx.accounts.nft_program.to_account_info(),
                    nft_program::cpi::accounts::MintNFT {
                        nft: // New NFT account (needs initialization),
                        nft_state: // NFT state account,
                        mint: nft_mint.to_account_info(),
                        token_account: nft_token_account.to_account_info(),
                        metadata_account: metadata_account.to_account_info(),
                        authority: ctx.accounts.authority.to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                        rent: ctx.accounts.rent.to_account_info(),
                        token_metadata_program: // Metaplex program,
                    }
                ),
                rarity
            )?;

            game_state.nft_count += 1;
        }
    }

    emit!(CommunityEventRewardEvent {
        player: player.pubkey,
        reward: match reward {
            EventReward::Seek { amount } => format!("{} $SEEK", amount),
            EventReward::NFT { rarity } => format!("NFT {:?}", rarity),
        },
    });

    Ok(())
}

fn is_admin(_pubkey: Pubkey) -> bool {
    true // Mock; use multisig
}

#[event]
pub struct CommunityEventRewardEvent {
    pub player: Pubkey,
    pub reward: String,
}
