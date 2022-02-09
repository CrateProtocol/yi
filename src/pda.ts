import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

import { YI_ADDRESSES } from ".";

/**
 * Finds the address of a YiToken.
 */
export const findYiTokenAddress = async (
  mint: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("YiToken"), mint.toBuffer()],
    YI_ADDRESSES.Yi
  );
};
