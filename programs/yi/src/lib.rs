//! Yi Token by Crate Protocol: the standard for auto-compounding single token staking pools.
//!
//! # Packages
//!
//! - NPM Package: [`@crateprotocol/yi`](https://www.npmjs.com/package/@crateprotocol/yi)
//! - Crates.io: [`yi`](https://crates.io/crates/yi)
//!
//! # Address
//!
//! The Yi program is deployed on `mainnet-beta` and `devnet` at [`YiiTopEnX2vyoWdXuG45ovDFYZars4XZ4w6td6RVTFm`](https://anchor.so/programs/YiiTopEnX2vyoWdXuG45ovDFYZars4XZ4w6td6RVTFm).
//!
//! # License
//!
//! Yi Token by Crate Protocol is licensed under the Affero General Public License, version 3.0.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![deny(clippy::unwrap_used)]

use anchor_lang::prelude::*;
use vipers::Validate;

mod instructions;
mod macros;
mod state;

pub use state::*;

use instructions::*;

declare_id!("YiiTopEnX2vyoWdXuG45ovDFYZars4XZ4w6td6RVTFm");

/// Number of millibps in 1.
pub const MILLIBPS_PER_WHOLE: u32 = 10_000 * 1_000;

/// The [yi] program.
#[program]
pub mod yi {
    use super::*;

    /// Creates a [YiToken].
    #[access_control(ctx.accounts.validate())]
    pub fn create_yi_token(ctx: Context<CreateYiToken>, bump: u8) -> ProgramResult {
        create_yi_token::handler(ctx, bump, 0, 0)
    }

    /// Creates a [YiToken] with fees which accrue to the [YiToken] holders.
    /// Fees cannot be modified after the [YiToken] is created.
    #[access_control(ctx.accounts.validate())]
    pub fn create_yi_token_with_fees(
        ctx: Context<CreateYiToken>,
        bump: u8,
        stake_fee_millibps: u32,
        unstake_fee_millibps: u32,
    ) -> ProgramResult {
        create_yi_token::handler(ctx, bump, stake_fee_millibps, unstake_fee_millibps)
    }

    /// Stakes underlying tokens for yiTokens.
    #[access_control(ctx.accounts.validate())]
    pub fn stake(ctx: Context<Stake>, amount: u64) -> ProgramResult {
        stake::handler(ctx, amount)
    }

    /// Unstakes yiTokens for their underlying tokens.
    #[access_control(ctx.accounts.validate())]
    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
        unstake::handler(ctx, amount)
    }
}

/// Errors.
#[error]
pub enum ErrorCode {
    #[msg("Decimal mismatch.")]
    DecimalMismatch,
    #[msg("Stake fee cannot exceed 100%.")]
    InvalidStakeFee,
    #[msg("Unstake fee cannot exceed 100%.")]
    InvalidUnstakeFee,
}
