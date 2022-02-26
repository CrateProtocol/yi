//! Yi Token by Crate Protocol: the primitive for auto-compounding single token staking pools.
//!
//! # About
//!
//! **Yi** is a Solana primitive for building single-sided, auto-compounded stake pools. It allows
//! projects to launch *Yi Tokens*, which are tokens convertible to and from an underlying token.
//!
//! Some example use cases include:
//!
//! - *Governance token staking.* Protocols may want their governance token to be staked, where fees are converted into the governance token. An example of this is the XSUSHI pool in Sushiswap.
//! - *Interest-bearing derivatives.* A protocol may collect revenue in a token that it wants users to stake, so it may pay fees out by compounding that token. An example of this is Anchor UST.
//!
//! # Usage
//!
//! First, create a Yi Token by invoking the [`yi::create_yi_token`] instruction. Then, anyone may stake
//! tokens into the pool via [`yi::stake`].
//!
//! To send auto-compounded rewards to the pool, deposit tokens to the [`YiToken::underlying_tokens`] token account.
//! This will increase the conversion rate of Yi Tokens to underlying tokens.
//!
//! To exit the pool, invoke [`yi::unstake`].
//!
//! ## Fees
//!
//! Yi Tokens may take stake or unstake fees. These fees cannot be changed after the construction of the Yi Token. Fees get distributed
//! to stakers within the Yi Token pool.
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
    pub fn create_yi_token(ctx: Context<CreateYiToken>) -> Result<()> {
        create_yi_token::handler(ctx, 0, 0)
    }

    /// Creates a [YiToken] with fees which accrue to the [YiToken] holders.
    /// Fees cannot be modified after the [YiToken] is created.
    #[access_control(ctx.accounts.validate())]
    pub fn create_yi_token_with_fees(
        ctx: Context<CreateYiToken>,
        stake_fee_millibps: u32,
        unstake_fee_millibps: u32,
    ) -> Result<()> {
        create_yi_token::handler(ctx, stake_fee_millibps, unstake_fee_millibps)
    }

    /// Stakes underlying tokens for yiTokens.
    #[access_control(ctx.accounts.validate())]
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }

    /// Unstakes yiTokens for their underlying tokens.
    #[access_control(ctx.accounts.validate())]
    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        unstake::handler(ctx, amount)
    }
}

/// Errors.
#[error_code]
pub enum ErrorCode {
    #[msg("Decimal mismatch.")]
    DecimalMismatch,
    #[msg("Stake fee cannot exceed 100%.")]
    InvalidStakeFee,
    #[msg("Unstake fee cannot exceed 100%.")]
    InvalidUnstakeFee,
}
