//! Struct definitions for accounts that hold state.
#![deny(missing_docs)]
#![deny(clippy::integer_arithmetic)]

use num_traits::ToPrimitive;

use crate::*;

/// A YiToken is an SPL Token which auto-compounds an underlying token.
///
/// It is a simplified version of the core Crate Protocol, optimized to minimize compute units.
#[account(zero_copy)]
#[derive(Debug, Default)]
pub struct YiToken {
    /// Mint of the [YiToken].
    pub mint: Pubkey,
    /// Bump seed.
    pub bump: u8,
    /// Padding.
    pub _padding: [u8; 7],

    /// The [anchor_spl::token::Mint] backing the [YiToken].
    pub underlying_token_mint: Pubkey,
    /// The [anchor_spl::token::TokenAccount] containing the staked tokens.
    pub underlying_tokens: Pubkey,

    /// The staking fee in thousands of bps.
    pub stake_fee_millibps: u32,
    /// The unstaking fee in thousands of bps.
    pub unstake_fee_millibps: u32,
}

impl YiToken {
    /// Calculates the number of [YiToken::underlying_token_mint] tokens to mint for the given amount of [YiToken]s.
    pub fn calculate_underlying_for_yitokens(
        &self,
        yitoken_amount: u64,
        total_underlying_tokens: u64,
        total_supply: u64,
    ) -> Option<u64> {
        if yitoken_amount == 0 {
            return Some(0);
        }
        // impossible to have more yitokens than the total supply.
        if yitoken_amount > total_supply {
            return None;
        }
        // if withdrawing all tokens, give the entire supply
        if yitoken_amount == total_supply {
            return Some(total_underlying_tokens);
        }
        let amt_no_fee = (yitoken_amount as u128)
            .checked_mul(total_underlying_tokens.into())?
            .checked_div(total_supply.into())?
            .to_u64()?;
        if self.unstake_fee_millibps == 0 {
            Some(amt_no_fee)
        } else {
            (amt_no_fee as u128)
                .checked_mul(self.unstake_fee_millibps.into())?
                .checked_div(MILLIBPS_PER_WHOLE.into())?
                .to_u64()
        }
    }

    /// Calculates the number of [YiToken]s to mint for the given amount of underlying tokens.
    pub fn calculate_yitokens_for_underlying(
        &self,
        underlying_amount: u64,
        total_underlying_tokens: u64,
        total_supply: u64,
    ) -> Option<u64> {
        if underlying_amount == 0 {
            return Some(0);
        }
        // if there are no tokens in the contract, it's 1:1
        if total_underlying_tokens == 0 {
            return Some(underlying_amount);
        }
        let amt_no_fee = (underlying_amount as u128)
            .checked_mul(total_supply.into())?
            .checked_div(total_underlying_tokens.into())?
            .to_u64()?;
        if self.stake_fee_millibps == 0 {
            Some(amt_no_fee)
        } else {
            (amt_no_fee as u128)
                .checked_mul(self.stake_fee_millibps.into())?
                .checked_div(MILLIBPS_PER_WHOLE.into())?
                .to_u64()
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::integer_arithmetic)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_calculate_yitokens_for_underlying_init() {
        let yi_token: YiToken = YiToken::default();

        let amount = yi_token
            .calculate_yitokens_for_underlying(700_000, 0, 0)
            .unwrap();
        assert_eq!(amount, 700_000);
    }

    #[test]
    fn test_calculate_yitokens_for_underlying_no_fees() {
        let yi_token: YiToken = YiToken::default();
        let amount = yi_token
            .calculate_yitokens_for_underlying(100_000, 700_000, 700_000)
            .unwrap();
        assert_eq!(amount, 100_000);
    }

    #[test]
    fn test_calculate_underlying_for_yitokens_init() {
        let yi_token: YiToken = YiToken::default();
        let amount = yi_token.calculate_underlying_for_yitokens(700_000, 0, 0);
        assert_eq!(amount, None);
    }

    #[test]
    fn test_calculate_underlying_for_yitokens_no_fees() {
        let yi_token: YiToken = YiToken::default();
        let amount = yi_token
            .calculate_underlying_for_yitokens(100_000, 700_000, 700_000)
            .unwrap();
        assert_eq!(amount, 100_000);
    }

    fn perform_test_cannot_increase_no_fees(
        initial_underlying_tokens: u64,
        initial_total_underlying_tokens: u64,
        initial_total_supply: u64,
        ratios: [f64; 32],
    ) {
        let yi_token: YiToken = YiToken::default();

        let mut my_yitokens = 0;
        let mut my_underlying_tokens = initial_underlying_tokens;

        let mut total_underlying_tokens: u64 = initial_total_underlying_tokens;
        let mut total_supply: u64 = initial_total_supply;

        for ratio in ratios {
            // unstake
            match ratio.partial_cmp(&0f64) {
                Some(std::cmp::Ordering::Less) => {
                    let underlying_stake_amount =
                        ((my_underlying_tokens as f64) * -ratio).to_u64().unwrap();
                    let mint_yitokens = yi_token
                        .calculate_yitokens_for_underlying(
                            underlying_stake_amount,
                            total_underlying_tokens,
                            total_supply,
                        )
                        .unwrap();

                    my_yitokens += mint_yitokens;
                    my_underlying_tokens -= underlying_stake_amount;

                    total_underlying_tokens += underlying_stake_amount;
                    total_supply += mint_yitokens;
                }
                Some(std::cmp::Ordering::Greater) => {
                    let yitoken_amount = ((my_yitokens as f64) * ratio).to_u64().unwrap();
                    let withdraw_underlying_tokens = yi_token
                        .calculate_underlying_for_yitokens(
                            yitoken_amount,
                            total_underlying_tokens,
                            total_supply,
                        )
                        .unwrap();

                    my_yitokens -= yitoken_amount;
                    my_underlying_tokens += withdraw_underlying_tokens;

                    total_underlying_tokens -= withdraw_underlying_tokens;
                    total_supply -= yitoken_amount;
                }
                _ => {
                    // do nothing
                }
            }
        }

        assert!(my_underlying_tokens <= initial_underlying_tokens);

        {
            // do a full unstake
            let final_yitokens = my_yitokens;
            let withdraw_underlying_tokens = yi_token
                .calculate_underlying_for_yitokens(
                    final_yitokens,
                    total_underlying_tokens,
                    total_supply,
                )
                .unwrap();

            my_yitokens -= final_yitokens;
            my_underlying_tokens += withdraw_underlying_tokens;

            total_underlying_tokens -= withdraw_underlying_tokens;
            total_supply -= my_yitokens;
        }

        assert_eq!(my_yitokens, 0);
        // user may have lost tokens due to rounding
        assert!(my_underlying_tokens <= initial_underlying_tokens);

        // pool may have gained tokens due to rounding
        assert!(total_underlying_tokens >= initial_total_underlying_tokens);
        assert!(total_supply >= initial_total_supply);
    }

    proptest! {
        #[test]
        fn cannot_increase_no_fees(
            initial_underlying_tokens in 0..=u32::MAX,
            initial_total_underlying_tokens in 0..=u32::MAX,
            initial_total_supply in 0..=u32::MAX,
            amounts in prop::array::uniform32(-1.0..=1.0)
        ) {
            perform_test_cannot_increase_no_fees(
                initial_underlying_tokens.into(),
                initial_total_underlying_tokens.into(),
                initial_total_supply.into(),
                amounts
            )
        }
    }
}
