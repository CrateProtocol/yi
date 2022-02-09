//! Processor for [yi::create_yi_token].

use crate::*;
use anchor_spl::token::{Mint, TokenAccount};
use vipers::prelude::*;

/// Accounts for [yi::create_yi_token].
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateYiToken<'info> {
    /// The [Mint] of the [YiToken].
    pub mint: Account<'info, Mint>,

    /// The [YiToken] to be created.
    #[account(
        init,
        seeds = [
            b"YiToken".as_ref(),
            mint.key().as_ref()
        ],
        bump = bump,
        payer = payer
    )]
    pub yi_token: AccountLoader<'info, YiToken>,

    /// [YiToken::underlying_token_mint].
    pub underlying_token_mint: Account<'info, Mint>,

    /// [YiToken::underlying_tokens].
    pub underlying_tokens: Account<'info, TokenAccount>,

    /// Payer.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// [System] program.
    pub system_program: Program<'info, System>,
}

impl<'info> CreateYiToken<'info> {
    fn create_yi_token(
        &mut self,
        bump: u8,
        stake_fee_millibps: u32,
        unstake_fee_millibps: u32,
    ) -> ProgramResult {
        let yi_token = &mut self.yi_token.load_init()?;
        yi_token.mint = self.mint.key();
        yi_token.bump = bump;
        yi_token.underlying_token_mint = self.underlying_token_mint.key();
        yi_token.underlying_tokens = self.underlying_tokens.key();

        invariant!(stake_fee_millibps <= MILLIBPS_PER_WHOLE, InvalidStakeFee);
        invariant!(
            unstake_fee_millibps <= MILLIBPS_PER_WHOLE,
            InvalidUnstakeFee
        );

        yi_token.stake_fee_millibps = stake_fee_millibps;
        yi_token.unstake_fee_millibps = unstake_fee_millibps;
        Ok(())
    }
}

pub fn handler(
    ctx: Context<CreateYiToken>,
    bump: u8,
    stake_fee_millibps: u32,
    unstake_fee_millibps: u32,
) -> ProgramResult {
    ctx.accounts
        .create_yi_token(bump, stake_fee_millibps, unstake_fee_millibps)
}

impl<'info> Validate<'info> for CreateYiToken<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys_eq!(self.mint.mint_authority.unwrap(), self.yi_token);
        assert_keys_eq!(self.mint.freeze_authority.unwrap(), self.yi_token);
        invariant!(self.mint.supply == 0);

        invariant!(
            self.underlying_token_mint.decimals == self.mint.decimals,
            DecimalMismatch
        );

        assert_is_zero_token_account!(self.underlying_tokens);
        assert_keys_eq!(self.underlying_tokens.owner, self.yi_token);
        assert_keys_eq!(self.underlying_tokens.mint, self.underlying_token_mint);
        Ok(())
    }
}
