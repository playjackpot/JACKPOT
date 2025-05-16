use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Seek<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub hide: Account<'info, Hide>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub seek_rewards_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Seek>, coordinates: (f64, f64)) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let hide = &ctx.accounts.hide;
    let player = &mut ctx.accounts.player;

    // Validate geolocation (off-chain, assume passed from backend)
    require!(
        is_within_radius(coordinates, hide.coordinates, 100.0),
        ErrorCode::InvalidLocation
    );

    // Charge $SEEK fees
    let start_fee = 2_500_000_000; // 2,500 $SEEK
    let find_fee = if game_state.year >= 8 { 2_000_000 } else { 1_000_000 }; // $1 or $2
    let cpi_accounts = Transfer {
        from: ctx.accounts.player_token_account.to_account_info(),
        to: ctx.accounts.seek_rewards_pool.to_account_info(),
        authority: ctx.accounts.player.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(
        CpiContext::new(cpi_program, cpi_accounts),
        start_fee + find_fee
    )?;

    // Burn 50% of find fee
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.player_token_account.mint.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            }
        ),
        find_fee / 2
    )?;

    // Distribute rewards
    let seek_reward = calculate_seek_reward(game_state.seek_rewards_pool);
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.seek_rewards_pool.to_account_info(),
                to: ctx.accounts.player_token_account.to_account_info(),
                authority: ctx.accounts.game_state.to_account_info(),
            }
        ),
        seek_reward
    )?;

    let sol_reward = calculate_sol_reward(ctx.accounts.sol_profit_wallet.key());
    **ctx.accounts.sol_profit_wallet.lamports.borrow_mut() -= sol_reward;
    **ctx.accounts.player.to_account_info().lamports.borrow_mut() += sol_reward;

    // Update state
    player.seeks += 1;
    update_rank(player, game_state)?;

    Ok(())
}

fn calculate_seek_reward(pool_balance: u64) -> u64 {
    if pool_balance >= 200_000_000_000_000 {
        25_000_000_000 // 25,000 $SEEK
    } else if pool_balance >= 150_000_000_000_000 {
        15_000_000_000 // 15,000 $SEEK
    } else if pool_balance >= 50_000_000_000_000 {
        5_000_000_000  // 5,000 $SEEK
    } else {
        2_500_000_000  // 2,500 $SEEK
    }
}

fn is_within_radius(player: (f64, f64), hide: (f64, f64), radius_m: f64) -> bool {
    // Simplified; actual haversine formula should be in backend
    true // Assume validated by backend
}

fn calculate_sol_reward(sol_profit_wallet: Pubkey) -> u64 {
    let balance = sol_profit_wallet.lamports();
    if balance > 20_000_000_000_000 {
        10_000_000
    } else if balance > 10_000_000_000_000 {
        5_000_000
    } else if balance > 5_000_000_000_000 {
        2_500_000
    } else {
        1_000_000
    }
}

fn update_rank(_player: &mut Player, _game_state: &mut GameState) -> Result<()> {
    // Implement leaderboard logic (simplified)
    Ok(())
}
use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, Token, TokenAccount, Transfer};
use jackpot_program::state::{GameState, Hide, Player, RewardData};
use nft_program::state::{PrizePalNFT, Rarity};

use crate::errors::*;

// Constants
const HINT_FEE: u64 = 100_000_000; // 100 $SEEK for hint
const BTC_AMOUNT: u64 = 1_000_000_000; // Mock 1 BTC in lamports
const USD_PER_SEEK: f64 = 25.0; // $25 start fee

#[derive(Accounts)]
#[instruction(coordinates: (f64, f64))]
pub struct Seek<'info> {
    #[account(mut)]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub hide: Account<'info, Hide>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub seek_rewards_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub sol_profit_wallet: SystemAccount<'info>,
    #[account(mut, constraint = nft.as_ref().map(|n| n.owner == player.pubkey).unwrap_or(true))]
    pub nft: Option<Account<'info, PrizePalNFT>>, // Optional NFT for hint
    #[account(mut)]
    pub nft_token_account: Option<Account<'info, TokenAccount>>, // NFT token account
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Hint {
    pub hint_type: HintType,
    pub data: String, // JSON string with hint details
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum HintType {
    BroadArea,      // Common: 1 km radius
    Direction,      // Rare: Direction + distance
    SmallArea,      // Epic: 100m radius
    Precise,        // Legendary: 10m radius
}

#[event]
pub struct HintEvent {
    pub player: Pubkey,
    pub hide: Pubkey,
    pub hint: String,
}

pub fn handler(ctx: Context<Seek>, coordinates: (f64, f64), use_hint: bool) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let hide = &ctx.accounts.hide;
    let player = &mut ctx.accounts.player;
    let seek_rewards_pool = &ctx.accounts.seek_rewards_pool;
    let player_token_account = &ctx.accounts.player_token_account;

    // Validate seek eligibility (3 hides for first seek, then 1:1)
    require!(
        player.seeks == 0 && player.hides >= 3 || player.seeks < player.hides - 2,
        ErrorCode::InvalidSeekSequence
    );

    // Validate geolocation (off-chain, assume passed from backend)
    require!(
        is_within_radius(coordinates, hide.coordinates, 100.0),
        ErrorCode::InvalidLocation
    );

    // Calculate $SEEK start fee pegged to $25
    let seek_price_usd = get_seek_price_usd(); // Mock oracle
    let start_fee = ((USD_PER_SEEK / seek_price_usd) * 1_000_000_000.0) as u64; // Convert to lamports
    let find_fee = if game_state.year >= 8 {
        (2.0 / seek_price_usd * 1_000_000_000.0) as u64 // $2
    } else {
        (1.0 / seek_price_usd * 1_000_000_000.0) as u64 // $1
    };
    let total_fee = start_fee + find_fee + if use_hint { HINT_FEE } else { 0 };

    // Validate $SEEK balance
    require!(
        player_token_account.amount >= total_fee,
        ErrorCode::InsufficientSeekBalance
    );

    // Transfer fees to $SEEK Rewards Pool
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: player_token_account.to_account_info(),
                to: seek_rewards_pool.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            }
        ),
        total_fee
    )?;

    // Burn 50% of find fee
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: player_token_account.mint.to_account_info(),
                to: player_token_account.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            }
        ),
        find_fee / 2
    )?;

    // Generate hint if requested
    let hint = if use_hint {
        let nft = ctx.accounts.nft.as_ref().ok_or(ErrorCode::NoNFTProvided)?;
        Some(generate_hint(nft, hide.coordinates, coordinates)?)
    } else {
        None
    };

    // Distribute rewards
    let mut rewards = RewardData { seek: 0, sol: 0, btc: 0 };

    // $SEEK reward
    rewards.seek = calculate_seek_reward(game_state.seek_rewards_pool);
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: seek_rewards_pool.to_account_info(),
                to: player_token_account.to_account_info(),
                authority: ctx.accounts.game_state.to_account_info(),
            }
        ),
        rewards.seek
    )?;
    game_state.seek_rewards_pool = game_state.seek_rewards_pool.saturating_sub(rewards.seek);

    // SOL reward
    rewards.sol = calculate_sol_reward(ctx.accounts.sol_profit_wallet.lamports());
    **ctx.accounts.sol_profit_wallet.lamports.borrow_mut() -= rewards.sol;
    **ctx.accounts.player.to_account_info().lamports.borrow_mut() += rewards.sol;

    // BTC reward (for annual BTC hide)
    if hide.treasure_box.unwrap_or(false) {
        // Assume KYC verified off-chain for seeker
        require!(verify_kyc(ctx.accounts.player.pubkey), ErrorCode::KYCNotVerified);
        rewards.btc = BTC_AMOUNT;
        // Transfer WBTC (requires CPI to btc_rewards_program)
        // Simplified: Assume handled by btc_rewards_program
        emit!(BTCFoundEvent {
            player: player.pubkey,
            hide: hide.key(),
            amount: rewards.btc,
        });
    }

    // Update player state
    player.seeks += 1;
    player.rewards.seek = player.rewards.seek.saturating_add(rewards.seek);
    player.rewards.sol = player.rewards.sol.saturating_add(rewards.sol);
    player.rewards.btc = player.rewards.btc.saturating_add(rewards.btc);
    update_rank(player, game_state)?;

    // Emit hint event for frontend
    if let Some(hint) = hint {
        emit!(HintEvent {
            player: ctx.accounts.player.pubkey,
            hide: ctx.accounts.hide.key(),
            hint: hint.data,
        });
    }

    Ok(())
}

fn calculate_seek_reward(pool_balance: u64) -> u64 {
    if pool_balance >= 200_000_000_000_000 {
        25_000_000_000 // 25,000 $SEEK
    } else if pool_balance >= 150_000_000_000_000 {
        15_000_000_000 // 15,000 $SEEK
    } else if pool_balance >= 50_000_000_000_000 {
        5_000_000_000  // 5,000 $SEEK
    } else if pool_balance >= 25_000_000_000_000 {
        2_500_000_000  // 2,500 $SEEK
    } else {
        1_000_000_000  // 1,000 $SEEK (Year 8+ minimum)
    }
}

fn calculate_sol_reward(sol_balance: u64) -> u64 {
    if sol_balance > 20_000_000_000_000 {
        10_000_000 // 0.01 SOL
    } else if sol_balance > 10_000_000_000_000 {
        5_000_000  // 0.005 SOL
    } else if sol_balance > 5_000_000_000_000 {
        2_500_000  // 0.0025 SOL
    } else {
        1_000_000  // 0.001 SOL
    }
}

fn is_within_radius(_player: (f64, f64), _hide: (f64, f64), _radius_m: f64) -> bool {
    // Simplified; actual haversine formula in backend
    true // Assume validated by backend
}

fn verify_kyc(_pubkey: Pubkey) -> bool {
    // Mock; integrate with KYC provider (e.g., Chainalysis)
    true // Assume verified
}

fn get_seek_price_usd() -> f64 {
    // Mock oracle; integrate with Pyth or Chainlink for real price
    0.01 // $0.01 per $SEEK as per whitepaper
}

fn generate_hint(nft: &PrizePalNFT, hide_coords: (f64, f64), player_coords: (f64, f64)) -> Result<Hint> {
    let hint_type = match nft.rarity {
        Rarity::Common => HintType::BroadArea,
        Rarity::Rare => HintType::Direction,
        Rarity::Epic => HintType::SmallArea,
        Rarity::Legendary => HintType::Precise,
    };

    let data = match hint_type {
        HintType::BroadArea => {
            // 1 km radius around hide
            serde_json::to_string(&serde_json::json!({
                "type": "broad_area",
                "center": {"lat": hide_coords.0, "lng": hide_coords.1},
                "radius_m": 1000
            }))?
        }
        HintType::Direction => {
            // Direction and approximate distance from player to hide
            let distance = mock_haversine(player_coords, hide_coords); // Mock distance
            let direction = calculate_direction(player_coords, hide_coords);
            serde_json::to_string(&serde_json::json!({
                "type": "direction",
                "direction": direction,
                "distance_m": distance
            }))?
        }
        HintType::SmallArea => {
            // 100m radius around hide
            serde_json::to_string(&serde_json::json!({
                "type": "small_area",
                "center": {"lat": hide_coords.0, "lng": hide_coords.1},
                "radius_m": 100
            }))?
        }
        HintType::Precise => {
            // Precise coordinates within 10m
            let offset = 0.00009; // ~10m in lat/lng
            serde_json::to_string(&serde_json::json!({
                "type": "precise",
                "center": {
                    "lat": hide_coords.0 + (rand::random::<f64>() - 0.5) * offset,
                    "lng": hide_coords.1 + (rand::random::<f64>() - 0.5) * offset
                },
                "radius_m": 10
            }))?
        }
    };

    Ok(Hint { hint_type, data })
}

fn mock_haversine(_player: (f64, f64), _hide: (f64, f64)) -> f64 {
    // Mock distance calculation; use backend haversine
    500.0 // ~500m
}

fn calculate_direction(_player: (f64, f64), _hide: (f64, f64)) -> String {
    // Mock direction; calculate based on coordinates
    "North".to_string() // Simplified
}

fn update_rank(_player: &mut Player, _game_state: &mut GameState) -> Result<()> {
    // Implement leaderboard logic
    Ok(())
}

#[event]
pub struct BTCFoundEvent {
    pub player: Pubkey,
    pub hide: Pubkey,
    pub amount: u64,
}
