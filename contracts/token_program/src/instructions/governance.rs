use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};

use crate::state::VestingSchedule;
use crate::errors::*;

#[derive(Accounts)]
pub struct CreateVestingSchedule<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 4
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseVestedTokens<'info> {
    #[account(mut)]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn create_vesting_schedule(ctx: Context<CreateVestingSchedule>, amount: u64, start_ts: i64, duration: i64) -> Result<()> {
    let vesting_schedule = &mut ctx.accounts.vesting_schedule;

    // Validate inputs
    require!(amount > 0, ErrorCode::InvalidVestingAmount);
    require!(duration > 0, ErrorCode::InvalidVestingDuration);
    require!(start_ts >= Clock::get()?.unix_timestamp, ErrorCode::InvalidVestingStart);

    // Transfer tokens to vesting account
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_token_account.to_account_info(),
                to: ctx.accounts.destination_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }
        ),
        amount
    )?;

    // Set vesting schedule
    vesting_schedule.beneficiary = ctx.accounts.authority.key();
    vesting_schedule.amount = amount;
    vesting_schedule.start_ts = start_ts;
    vesting_schedule.duration = duration;
    vesting_schedule.released = 0;

    emit!(VestingCreatedEvent {
        beneficiary: ctx.accounts.authority.key(),
        amount,
        start_ts,
        duration,
    });

    Ok(())
}

pub fn release_vested_tokens(ctx: Context<ReleaseVestedTokens>) -> Result<()> {
    let vesting_schedule = &mut ctx.accounts.vesting_schedule;
    let current_ts = Clock::get()?.unix_timestamp;

    // Calculate vested amount
    let elapsed = (current_ts - vesting_schedule.start_ts).max(0) as u64;
    let total_duration = vesting_schedule.duration as u64;
    let vested_amount = if elapsed >= total_duration {
        vesting_schedule.amount
    } else {
        (vesting_schedule.amount * elapsed) / total_duration
    };
    let releasable = vested_amount.saturating_sub(vesting_schedule.released);

    // Validate
    require!(releasable > 0, ErrorCode::NoVestedTokensAvailable);

    // Transfer releasable tokens
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_token_account.to_account_info(),
                to: ctx.accounts.destination_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            }
        ),
        releasable
    )?;

    // Update state
    vesting_schedule.released += releasable;

    emit!(TokensReleasedEvent {
        beneficiary: vesting_schedule.beneficiary,
        amount: releasable,
    });

    Ok(())
}

#[event]
pub struct VestingCreatedEvent {
    pub beneficiary: Pubkey,
    pub amount: u64,
    pub start_ts: i64,
    pub duration: i64,
}

#[event]
pub struct TokensReleasedEvent {
    pub beneficiary: Pubkey,
    pub amount: u64,
}
