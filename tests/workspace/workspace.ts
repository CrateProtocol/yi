import * as anchor from "@project-serum/anchor";
import { makeSaberProvider } from "@saberhq/anchor-contrib";
import { chaiSolana } from "@saberhq/chai-solana";
import chai from "chai";

import type { YiPrograms } from "../../src";
import { YiSDK } from "../../src";

chai.use(chaiSolana);

export type Workspace = YiPrograms;

export const makeSDK = (): YiSDK => {
  const anchorProvider = anchor.Provider.env();
  anchor.setProvider(anchorProvider);
  const provider = makeSaberProvider(anchorProvider);
  return YiSDK.load({
    provider,
  });
};
