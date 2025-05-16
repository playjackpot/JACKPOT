use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use btc_rewards_program::{self, state::*};

declare_id!("YourBTCProgramIDHere");

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::system_instruction;

    #[test]
    fn test_annual_btc_hide() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "btc_rewards_program",
            program_id,
            processor!(btc_rewards_program::entry),
        );

        // Setup accounts
        let wallet_state = Pubkey::new_unique();
        let btc_rewards_wallet = Pubkey::new_unique();
        let btc_drop_wallet = Pubkey::new_unique();
        let jackpot_program = Pubkey::new_unique();
        let hide = Pubkey::new_unique();
        let authority = Pubkey::new_unique();

        // Initialize wallet state
        let mut wallet_state_data = BTCWalletState {
            btc_rewards_wallet,
            btc_drop_wallet,
            sol_profit_wallet: Pubkey::new_unique(),
            fees_collected: 0,
            year: 1,
            annual_hide_created: false,
            annual_hide: None,
            btc_price_usd: 60_000_000_000_000, // Mock $60,000
        };

        // Simulate annual BTC hide
        let instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(wallet_state, false),
                AccountMeta::new(btc_rewards_wallet, false),
                AccountMeta::new(btc_drop_wallet, false),
                AccountMeta::new_readonly(jackpot_program, false),
                AccountMeta::new(hide, false),
                AccountMeta::new(authority, true),
                AccountMeta::new_readonly(anchor_spl::token::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(Sysvar::Rent::id(), false),
            ],
            data: anchor_lang::InstructionData::data(&btc_rewards_program::AnnualBTCHide {
                coordinates: (37.78825, -122.4324),
            }),
        };

        // Mock accounts
        let mut ctx = program_test.context.borrow_mut();
        ctx.set_account(&btc_rewards_wallet, &Account::new_data(
            2_000_000_000,
            &TokenAccount { amount: 2_000_000_000, ..Default::default() },
            &anchor_spl::token::ID,
        )?);
        ctx.set_account(&btc_drop_wallet, &Account::new_data(
            0,
            &TokenAccount { amount: 0, ..Default::default() },
            &anchor_spl::token::ID,
        )?);

        // Process instruction
        let result = ctx.process_instruction(instruction);
        assert!(result.is_ok());

        // Verify state
        assert!(wallet_state_data.annual_hide_created);
        assert_eq!(wallet_state_data.annual_hide, Some(hide));
    }
}
