//! Processor for [yi::stake].

use anchor_spl::token::{self, Mint, Token, TokenAccount};
use vipers::prelude::*;

use crate::*;

/// Accounts for [yi::stake].
#[derive(Accounts)]
pub struct Stake<'info> {
    /// The [YiToken] to stake tokens into.
    pub yi_token: AccountLoader<'info, YiToken>,

    /// [YiToken::mint]. [Mint] of the [YiToken].
    #[account(mut)]
    pub yi_mint: Account<'info, Mint>,

    /// Tokens to be staked into the [YiToken].
    #[account(mut)]
    pub source_tokens: Account<'info, TokenAccount>,
    /// The [TokenAccount::owner] of [Self::source_tokens].
    pub source_authority: Signer<'info>,
    /// [YiToken::underlying_tokens].
    #[account(mut)]
    pub yi_underlying_tokens: Account<'info, TokenAccount>,

    /// The [TokenAccount] receiving the minted [YiToken]s.
    #[account(mut)]
    pub destination_yi_tokens: Box<Account<'info, TokenAccount>>,

    /// The [token] program.
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    fn deposit_underlying(&self, amount: u64) -> ProgramResult {
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::Transfer {
                    from: self.source_tokens.to_account_info(),
                    to: self.yi_underlying_tokens.to_account_info(),
                    authority: self.source_authority.to_account_info(),
                },
            ),
            amount,
        )
    }

    fn mint_yi_tokens(&self, amount: u64) -> ProgramResult {
        let yi_token = self.yi_token.load()?;
        let signer_seeds: &[&[&[u8]]] = yitoken_seeds!(yi_token);
        token::mint_to(
            CpiContext::new(
                self.token_program.to_account_info(),
                token::MintTo {
                    mint: self.yi_mint.to_account_info(),
                    to: self.destination_yi_tokens.to_account_info(),
                    authority: self.yi_token.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            amount,
        )
    }

    pub(crate) fn stake(&self, underlying_amount: u64) -> ProgramResult {
        let yi_token = self.yi_token.load()?;
        self.deposit_underlying(underlying_amount)?;
        let mint_amount = unwrap_int!(yi_token.calculate_yitokens_for_underlying(
            underlying_amount,
            self.yi_underlying_tokens.amount,
            self.yi_mint.supply
        ));
        self.mint_yi_tokens(mint_amount)?;
        Ok(())
    }
}

pub fn handler(ctx: Context<Stake>, underlying_amount: u64) -> ProgramResult {
    // short circuit if no amount specified
    if underlying_amount == 0 {
        return Ok(());
    }
    ctx.accounts.stake(underlying_amount)
}

impl<'info> Validate<'info> for Stake<'info> {
    fn validate(&self) -> ProgramResult {
        let yi_token = self.yi_token.load()?;
        assert_keys_eq!(self.yi_mint, yi_token.mint);
        assert_keys_eq!(self.source_tokens.mint, yi_token.underlying_token_mint);
        assert_keys_eq!(self.source_authority, self.source_tokens.owner);
        assert_keys_eq!(self.yi_underlying_tokens, yi_token.underlying_tokens);

        assert_keys_eq!(self.destination_yi_tokens.mint, yi_token.mint);
        Ok(())
    }
}
