import { buildCoderMap } from "@saberhq/anchor-contrib";
import { PublicKey } from "@solana/web3.js";

import type { YiProgram, YiTypes } from "./programs";
import { YiJSON } from "./programs";

/**
 * Yi program types.
 */
export interface YiPrograms {
  Yi: YiProgram;
}

/**
 * Yi addresses.
 */
export const YI_ADDRESSES = {
  Yi: new PublicKey("YiiTopEnX2vyoWdXuG45ovDFYZars4XZ4w6td6RVTFm"),
};

/**
 * Program IDLs.
 */
export const YI_IDLS = {
  Yi: YiJSON,
};

/**
 * Coders.
 */
export const YI_CODERS = buildCoderMap<{
  Yi: YiTypes;
}>(YI_IDLS, YI_ADDRESSES);
