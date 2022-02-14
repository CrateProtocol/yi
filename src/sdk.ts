import { newProgramMap } from "@saberhq/anchor-contrib";
import type {
  AugmentedProvider,
  Provider,
  TransactionEnvelope,
} from "@saberhq/solana-contrib";
import { SolanaAugmentedProvider } from "@saberhq/solana-contrib";
import type { Token, TokenAmount, u64 } from "@saberhq/token-utils";
import {
  createInitMintInstructions,
  getOrCreateATA,
  getOrCreateATAs,
  TOKEN_PROGRAM_ID,
} from "@saberhq/token-utils";
import type { PublicKey, Signer } from "@solana/web3.js";
import { Keypair, SystemProgram } from "@solana/web3.js";

import type { YiPrograms } from ".";
import { findYiTokenAddress } from ".";
import { YI_ADDRESSES, YI_IDLS } from "./constants";

/**
 * Yi SDK.
 */
export class YiSDK {
  constructor(
    readonly provider: AugmentedProvider,
    readonly programs: YiPrograms
  ) {}

  /**
   * Creates a new instance of the SDK with the given keypair.
   */
  withSigner(signer: Signer): YiSDK {
    return YiSDK.load({
      provider: this.provider.withSigner(signer),
    });
  }

  /**
   * Loads the SDK.
   * @returns
   */
  static load({ provider }: { provider: Provider }): YiSDK {
    const programs: YiPrograms = newProgramMap<YiPrograms>(
      provider,
      YI_IDLS,
      YI_ADDRESSES
    );
    return new YiSDK(new SolanaAugmentedProvider(provider), programs);
  }

  /**
   * Creates a new Yi Token.
   * @returns
   */
  async createYiToken({
    underlyingToken,
    stakeFeeMillibps = 0,
    unstakeFeeMillibps = 0,
    mintKP = Keypair.generate(),
    payer = this.provider.wallet.publicKey,
  }: {
    underlyingToken: Token;
    stakeFeeMillibps?: number;
    unstakeFeeMillibps?: number;
    /**
     * Keypair of the mint of the YiToken.
     */
    mintKP?: Signer;
    payer?: PublicKey;
  }): Promise<{
    tx: TransactionEnvelope;
    mint: PublicKey;
    yiToken: PublicKey;
  }> {
    const [yiToken, bump] = await findYiTokenAddress(mintKP.publicKey);
    const underlyingTokens = await getOrCreateATA({
      provider: this.provider,
      mint: underlyingToken.mintAccount,
      owner: yiToken,
    });
    const initMintTX = await createInitMintInstructions({
      provider: this.provider,
      mintKP,
      decimals: underlyingToken.decimals,
      mintAuthority: yiToken,
      freezeAuthority: yiToken,
    });
    return {
      yiToken,
      mint: mintKP.publicKey,
      tx: initMintTX.combine(
        this.provider.newTX([
          underlyingTokens.instruction,
          this.programs.Yi.instruction.createYiTokenWithFees(
            bump,
            stakeFeeMillibps,
            unstakeFeeMillibps,
            {
              accounts: {
                mint: mintKP.publicKey,
                yiToken,
                underlyingTokenMint: underlyingToken.mintAccount,
                underlyingTokens: underlyingTokens.address,
                payer,
                systemProgram: SystemProgram.programId,
              },
            }
          ),
        ])
      ),
    };
  }

  /**
   * Stakes underlying tokens for Yi tokens.
   * @returns
   */
  async stake({
    yiTokenMint,
    amount,
    authority = this.provider.wallet.publicKey,
  }: {
    /**
     * Mint of the Yi token.
     */
    yiTokenMint: PublicKey;
    /**
     * Amount of underlying tokens to stake.
     */
    amount: u64;
    /**
     * Authority staking tokens.
     */
    authority?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const [yiToken] = await findYiTokenAddress(yiTokenMint);
    const yiTokenData = await this.programs.Yi.account.yiToken.fetch(yiToken);
    const authorityATAs = await getOrCreateATAs({
      provider: this.provider,
      mints: {
        underlying: yiTokenData.underlyingTokenMint,
        yi: yiTokenData.mint,
      },
      owner: authority,
    });
    return this.provider.newTX([
      authorityATAs.createAccountInstructions.yi,
      this.programs.Yi.instruction.stake(amount, {
        accounts: {
          yiToken,
          yiMint: yiTokenMint,
          sourceTokens: authorityATAs.accounts.underlying,
          sourceAuthority: authority,
          yiUnderlyingTokens: yiTokenData.underlyingTokens,
          destinationYiTokens: authorityATAs.accounts.yi,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
      }),
    ]);
  }

  /**
   * Unstakes Yi tokens.
   * @returns
   */
  async unstake({
    amount,
    authority = this.provider.wallet.publicKey,
  }: {
    /**
     * Yi Token Amount
     */
    amount: TokenAmount;
    /**
     * Authority unstaking tokens.
     */
    authority?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const [yiToken] = await findYiTokenAddress(amount.token.mintAccount);
    const yiTokenData = await this.programs.Yi.account.yiToken.fetch(yiToken);
    const authorityATAs = await getOrCreateATAs({
      provider: this.provider,
      mints: {
        underlying: yiTokenData.underlyingTokenMint,
        yi: yiTokenData.mint,
      },
      owner: authority,
    });
    return this.provider.newTX([
      ...authorityATAs.instructions,
      this.programs.Yi.instruction.unstake(amount.toU64(), {
        accounts: {
          yiToken,
          yiMint: amount.token.mintAccount,
          sourceYiTokens: authorityATAs.accounts.yi,
          sourceAuthority: authority,
          yiUnderlyingTokens: yiTokenData.underlyingTokens,
          destinationUnderlyingTokens: authorityATAs.accounts.underlying,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
      }),
    ]);
  }
}
