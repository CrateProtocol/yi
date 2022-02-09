import { assertTXSuccess, chaiSolana } from "@saberhq/chai-solana";
import {
  getTokenAccount,
  SPLToken,
  Token,
  TOKEN_PROGRAM_ID,
  TokenAmount,
  TokenAugmentedProvider,
  u64,
} from "@saberhq/token-utils";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";
import * as chai from "chai";

import { findYiTokenAddress } from "../src/pda";
import type { YiTokenData } from "../src/programs/yi";
import { makeSDK } from "./workspace";

chai.use(chaiSolana);

describe("YiToken", () => {
  const sdk = makeSDK();
  const provider = new TokenAugmentedProvider(sdk.provider);

  it("create yiToken", async () => {
    const underlyingToken = await provider.createToken();
    const yiMintKP = Keypair.generate();

    const {
      yiToken,
      mint,
      tx: createYiTokenTX,
    } = await sdk.createYiToken({
      underlyingToken,
      mintKP: yiMintKP,
    });
    await assertTXSuccess(createYiTokenTX);

    const [expectedYiToken] = await findYiTokenAddress(yiMintKP.publicKey);
    expect(yiToken).to.eqAddress(expectedYiToken);
    expect(mint).to.eqAddress(yiMintKP.publicKey);
  });

  describe("feeless yiToken", () => {
    let yiToken: Token;
    let underlyingToken: Token;
    let yiTokenData: YiTokenData;

    beforeEach("create yiToken", async () => {
      underlyingToken = await provider.createToken();
      const {
        mint,
        tx: createYiTokenTX,
        yiToken: yiTokenKey,
      } = await sdk.createYiToken({
        underlyingToken,
      });
      await assertTXSuccess(createYiTokenTX);
      yiToken = Token.fromMint(mint, 6);
      yiTokenData = await sdk.programs.Yi.account.yiToken.fetch(yiTokenKey);
    });

    it("stake/unstake", async () => {
      const mintAmount = new u64(1_000_000);

      const to = await provider.getOrCreateATA({
        mint: underlyingToken.mintAccount,
      });

      await assertTXSuccess(
        provider.newTX([
          to.instruction,
          SPLToken.createMintToInstruction(
            TOKEN_PROGRAM_ID,
            underlyingToken.mintAccount,
            to.address,
            provider.walletKey,
            [],
            mintAmount
          ),
        ])
      );

      const yiATA = await provider.getOrCreateATA({
        mint: yiToken.mintAccount,
        owner: provider.walletKey,
      });
      await assertTXSuccess(provider.newTX([yiATA.instruction]));

      // before
      {
        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(new u64(1_000_000));
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(0));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(0));
      }

      const stakeTX = await sdk.stake({
        yiTokenMint: yiToken.mintAccount,
        amount: new u64(200_000),
      });
      await assertTXSuccess(stakeTX);

      // after stake
      {
        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(new u64(800_000));
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(200_000));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(200_000));
      }

      const unstakeTX = await sdk.unstake({
        amount: new TokenAmount(yiToken, 200_000),
      });
      await assertTXSuccess(unstakeTX);

      // after unstake
      {
        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(new u64(1_000_000));
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(0));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(0));
      }
    });

    it("compound direct tokens", async () => {
      const mintAmount = new u64(1_000_000);
      const compoundAmount = new u64(500_000);

      const to = await provider.getOrCreateATA({
        mint: underlyingToken.mintAccount,
      });

      await assertTXSuccess(
        provider.newTX([
          to.instruction,
          SPLToken.createMintToInstruction(
            TOKEN_PROGRAM_ID,
            underlyingToken.mintAccount,
            to.address,
            provider.walletKey,
            [],
            mintAmount
          ),
        ])
      );

      const yiATA = await provider.getOrCreateATA({
        mint: yiToken.mintAccount,
        owner: provider.walletKey,
      });
      await assertTXSuccess(provider.newTX([yiATA.instruction]));

      // before
      {
        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(new u64(1_000_000));
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(0));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(0));
      }

      // stake tokens
      {
        const stakeTX = await sdk.stake({
          yiTokenMint: yiToken.mintAccount,
          amount: new u64(200_000),
        });
        await assertTXSuccess(stakeTX);

        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(new u64(800_000));
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(200_000));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(200_000));
      }

      // mint more tokens
      {
        await assertTXSuccess(
          provider.mintToAccount({
            amount: new TokenAmount(underlyingToken, compoundAmount),
            destination: yiTokenData.underlyingTokens,
          })
        );

        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(new u64(800_000));
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(200_000));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(700_000));
      }

      // unstake half of tokens
      {
        const unstakeTX = await sdk.unstake({
          amount: new TokenAmount(yiToken, 100_000),
        });
        await assertTXSuccess(unstakeTX);

        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(new u64(1_150_000));
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(100_000));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(350_000));
      }

      // unstake with a round down
      {
        const unstakeTX = await sdk.unstake({
          amount: new TokenAmount(yiToken, 33_333),
        });
        await assertTXSuccess(unstakeTX);

        const destination = await getTokenAccount(provider, to.address);
        expect(destination.amount).to.bignumber.eq(
          new u64(1_150_000 + 116_665)
        );
        const yiAccount = await getTokenAccount(provider, yiATA.address);
        expect(yiAccount.amount).to.bignumber.eq(new u64(66_667));
        const yiUnderlying = await getTokenAccount(
          provider,
          yiTokenData.underlyingTokens
        );
        expect(yiUnderlying.amount).to.bignumber.eq(new u64(350_000 - 116_665));
      }
    });
  });
});
