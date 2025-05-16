use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo};

declare_id!("YourTokenProgramIDHere");

#[program]
pub mod token_program {
    use super::*;

    pub fn initialize_token(ctx: Context<InitializeToken>) -> Result<()> {
        // Initialize mint with authority
        let cpi_accounts = anchor_spl::token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        anchor_spl::token::initialize_mint(
            CpiContext::new(cpi_program, cpi_accounts),
            9, // 9 decimals
            &ctx.accounts.authority.key(),
            Some(&ctx.accounts.authority.key()),
        )?;
        Ok(())
    }

    pub fn mint_initial_supply(ctx: Context<MintInitialSupply>, amount: u64) -> Result<()> {
        // Mint initial 1B $SEEK
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        anchor_spl::token::mint_to(
            CpiContext::new(cpi_program, cpi_accounts),
            amount
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintInitialSupply<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo};

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;
use errors::*;

declare_id!("YourTokenProgramIDHere");

#[program]
pub mod token_program {
    use super::*;

    pub fn initialize_token(ctx: Context<InitializeToken>) -> Result<()> {
        instructions::initialize_token::handler(ctx)
    }

    pub fn mint_initial_supply(ctx: Context<MintInitialSupply>, amount: u64) -> Result<()> {
        instructions::mint_initial_supply::handler(ctx, amount)
    }

    pub fn create_vesting_schedule(ctx: Context<CreateVestingSchedule>, amount: u64, start_ts: i64, duration: i64) -> Result<()> {
        instructions::governance::create_vesting_schedule(ctx, amount, start_ts, duration)
    }

    pub fn release_vested_tokens(ctx: Context<ReleaseVestedTokens>) -> Result<()> {
        instructions::governance::release_vested_tokens(ctx)
    }
}
