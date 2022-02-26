//! Processor for [yi::unstake].

use crate::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use vipers::prelude::*;

/// Accounts for [yi::unstake].
#[derive(Accounts)]
pub struct Unstake<'info> {
    /// The [YiToken] to unstake tokens from.
    pub yi_token: AccountLoader<'info, YiToken>,

    /// [YiToken::mint]. [Mint] of the [YiToken].
    #[account(mut)]
    pub yi_mint: Account<'info, Mint>,
    /// [YiToken]s to be burned.
    #[account(mut)]
    pub source_yi_tokens: Account<'info, TokenAccount>,
    /// The [TokenAccount::owner] of [Self::source_yi_tokens].
    pub source_authority: Signer<'info>,

    /// [YiToken::underlying_tokens].
    #[account(mut)]
    pub yi_underlying_tokens: Account<'info, TokenAccount>,
    /// The [TokenAccount] receiving the underlying tokens.
    #[account(mut)]
    pub destination_underlying_tokens: Box<Account<'info, TokenAccount>>,

    /// The [token] program.
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    fn withdraw_underlying(&self, amount: u64) -> Result<()> {
        let yi_token = self.yi_token.load()?;
        let signer_seeds: &[&[&[u8]]] = yitoken_seeds!(yi_token);
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::Transfer {
                    from: self.yi_underlying_tokens.to_account_info(),
                    to: self.destination_underlying_tokens.to_account_info(),
                    authority: self.yi_token.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            amount,
        )
    }

    fn burn_yi_tokens(&self, yitoken_amount: u64) -> Result<()> {
        token::burn(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::Burn {
                    mint: self.yi_mint.to_account_info(),
                    to: self.source_yi_tokens.to_account_info(),
                    authority: self.source_authority.to_account_info(),
                },
            ),
            yitoken_amount,
        )
    }

    pub(crate) fn unstake(&self, yitoken_amount: u64) -> Result<()> {
        let yi_token = self.yi_token.load()?;

        self.burn_yi_tokens(yitoken_amount)?;
        let withdraw_amount = unwrap_int!(yi_token.calculate_underlying_for_yitokens(
            yitoken_amount,
            self.yi_underlying_tokens.amount,
            self.yi_mint.supply
        ));
        self.withdraw_underlying(withdraw_amount)?;
        Ok(())
    }
}

pub fn handler(ctx: Context<Unstake>, yitoken_amount: u64) -> Result<()> {
    // short circuit if no amount specified
    if yitoken_amount == 0 {
        return Ok(());
    }
    ctx.accounts.unstake(yitoken_amount)
}

impl<'info> Validate<'info> for Unstake<'info> {
    fn validate(&self) -> Result<()> {
        let yi_token = self.yi_token.load()?;
        assert_keys_eq!(self.yi_mint, yi_token.mint);
        assert_keys_eq!(self.source_yi_tokens.mint, self.yi_mint);
        assert_keys_eq!(self.source_authority, self.source_yi_tokens.owner);

        assert_keys_eq!(self.yi_underlying_tokens, yi_token.underlying_tokens);
        assert_keys_eq!(
            self.destination_underlying_tokens.mint,
            yi_token.underlying_token_mint
        );
        Ok(())
    }
}
