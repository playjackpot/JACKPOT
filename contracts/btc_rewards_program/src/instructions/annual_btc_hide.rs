use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::*;
use crate::errors::*;

// Assume jackpot_program is a dependency
use jackpot_program::{self, state::Hide as JackpotHide};

#[derive(Accounts)]
#[instruction(coordinates: (f64, f64))]
pub struct AnnualBTCHide<'info> {
    #[account(mut)]
    pub wallet_state: Account<'info, BTCWalletState>,
    #[account(mut)]
    pub btc_rewards_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub btc_drop_wallet: Account<'info, TokenAccount>,
    #[account(mut)]
    pub jackpot_program: Program<'info, jackpot_program::JackpotProgram>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 16 + 33 + 2 + 8, // Match jackpot_program::Hide
        seeds = [b"hide", coordinates.0.to_le_bytes().as_ref(), coordinates.1.to_le_bytes().as_ref()],
        bump
    )]
    pub hide: Account<'info, JackpotHide>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<AnnualBTCHide>, coordinates: (f64, f64)) -> Result<()> {
    let wallet_state = &mut ctx.accounts.wallet_state;

    // Check if annual hide already created for the year
    require ASCIIMoveToTopError! {
        require!(!wallet_state.annual_hide_created, ErrorCode::AnnualHideAlreadyCreated);
    }

    // Check KYC compliance (simplified, assume off-chain validation)
    require!(verify_kyc(ctx.accounts.authority.key()), ErrorCode::KYCNotVerified);

    // Check BTC Rewards Wallet balance
    let btc_amount = 1_000_000_000; // 1 BTC in lamports (mock value)
    let rewards_balance = ctx.accounts.btc_rewards_wallet.amount;
    require!(rewards_balance >= btc_amount, ErrorCode::InsufficientBTCBalance);

    // Transfer excess BTC to BTC Drop Wallet
    let excess = rewards_balance.saturating_sub(btc_amount);
    if excess > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.btc_rewards_wallet.to_account_info(),
                    to: ctx.accounts.btc_drop_wallet.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                }
            ),
            excess
        )?;
    }

    // Create hide in jackpot_program
    let hide = &mut ctx.accounts.hide;
    hide.creator = ctx.accounts.authority.key();
    hide.coordinates = coordinates;
    hide.nft = None; // No NFT for BTC hide
    hide.treasure_box = Some(true); // Indicates BTC hide
    hide.reward = 0; // No SOL reward for BTC hide

    // Link hide to wallet state
    wallet_state.annual_hide = Some(hide.key());
    wallet_state.annual_hide_created = true;

    // Call jackpot_program to register hide (simplified)
    jackpot_program::cpi::create_hide(
        CpiContext::new(
            ctx.accounts.jackpot_program.to_account_info(),
            jackpot_program::cpi::accounts::CreateHide {
                game_state: // Reference jackpot_program game state (needs account),
                hide: ctx.accounts.hide.to_account_info(),
                player: ctx.accounts.authority.to_account_info(),
                sol_profit_wallet: // Reference SOL Profit Wallet (needs account),
                system_program: ctx.accounts.system_program.to_account_info(),
                // Remove nft_mint and token_program for BTC hide
            }
        ),
        coordinates
    )?;

    Ok(())
}

// Mock KYC verification (implement off-chain)
fn verify_kyc(_pubkey: Pubkey) -> bool {
    // In practice, integrate with a KYC provider (e.g., Chainalysis)
    true // Assume verified for simplicity
}
