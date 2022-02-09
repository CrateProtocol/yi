import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { YiIDL } from "../idls/yi";

export * from "../idls/yi";

export type YiTypes = AnchorTypes<
  YiIDL,
  {
    yiToken: YiTokenData;
  }
>;

type Accounts = YiTypes["Accounts"];

export type YiTokenData = Accounts["YiToken"];

export type YiProgram = YiTypes["Program"];
