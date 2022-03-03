import { StdFee } from "@cosmjs/launchpad";
import { Registry, OfflineSigner, EncodeObject } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgDeletePoolRanking } from "./types/qoracle/tx";
import { MsgDeletePoolSpotPrice } from "./types/qoracle/tx";
import { MsgUpdatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgCreatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgCreatePoolRanking } from "./types/qoracle/tx";
import { MsgDeletePoolPosition } from "./types/qoracle/tx";
import { MsgDeletePoolInfo } from "./types/qoracle/tx";
import { MsgCreatePoolPosition } from "./types/qoracle/tx";
import { MsgUpdatePoolPosition } from "./types/qoracle/tx";
import { MsgUpdatePoolRanking } from "./types/qoracle/tx";
import { MsgCreatePoolInfo } from "./types/qoracle/tx";
import { MsgUpdatePoolInfo } from "./types/qoracle/tx";
export declare const MissingWalletError: Error;
export declare const registry: Registry;
interface TxClientOptions {
    addr: string;
}
interface SignAndBroadcastOptions {
    fee: StdFee;
    memo?: string;
}
declare const txClient: (wallet: OfflineSigner, { addr: addr }?: TxClientOptions) => Promise<{
    signAndBroadcast: (msgs: EncodeObject[], { fee, memo }?: SignAndBroadcastOptions) => any;
    msgDeletePoolRanking: (data: MsgDeletePoolRanking) => EncodeObject;
    msgDeletePoolSpotPrice: (data: MsgDeletePoolSpotPrice) => EncodeObject;
    msgUpdatePoolSpotPrice: (data: MsgUpdatePoolSpotPrice) => EncodeObject;
    msgCreatePoolSpotPrice: (data: MsgCreatePoolSpotPrice) => EncodeObject;
    msgCreatePoolRanking: (data: MsgCreatePoolRanking) => EncodeObject;
    msgDeletePoolPosition: (data: MsgDeletePoolPosition) => EncodeObject;
    msgDeletePoolInfo: (data: MsgDeletePoolInfo) => EncodeObject;
    msgCreatePoolPosition: (data: MsgCreatePoolPosition) => EncodeObject;
    msgUpdatePoolPosition: (data: MsgUpdatePoolPosition) => EncodeObject;
    msgUpdatePoolRanking: (data: MsgUpdatePoolRanking) => EncodeObject;
    msgCreatePoolInfo: (data: MsgCreatePoolInfo) => EncodeObject;
    msgUpdatePoolInfo: (data: MsgUpdatePoolInfo) => EncodeObject;
}>;
interface QueryClientOptions {
    addr: string;
}
declare const queryClient: ({ addr: addr }?: QueryClientOptions) => Promise<Api<unknown>>;
export { txClient, queryClient, };
