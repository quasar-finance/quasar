import { StdFee } from "@cosmjs/launchpad";
import { Registry, OfflineSigner, EncodeObject } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgDeletePoolSpotPrice } from "./types/qoracle/tx";
import { MsgCreatePoolPosition } from "./types/qoracle/tx";
import { MsgDeletePoolRanking } from "./types/qoracle/tx";
import { MsgCreatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgDeletePoolInfo } from "./types/qoracle/tx";
import { MsgUpdatePoolRanking } from "./types/qoracle/tx";
import { MsgDeletePoolPosition } from "./types/qoracle/tx";
import { MsgUpdatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgCreatePoolRanking } from "./types/qoracle/tx";
import { MsgCreatePoolInfo } from "./types/qoracle/tx";
import { MsgUpdatePoolInfo } from "./types/qoracle/tx";
import { MsgUpdatePoolPosition } from "./types/qoracle/tx";
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
    msgDeletePoolSpotPrice: (data: MsgDeletePoolSpotPrice) => EncodeObject;
    msgCreatePoolPosition: (data: MsgCreatePoolPosition) => EncodeObject;
    msgDeletePoolRanking: (data: MsgDeletePoolRanking) => EncodeObject;
    msgCreatePoolSpotPrice: (data: MsgCreatePoolSpotPrice) => EncodeObject;
    msgDeletePoolInfo: (data: MsgDeletePoolInfo) => EncodeObject;
    msgUpdatePoolRanking: (data: MsgUpdatePoolRanking) => EncodeObject;
    msgDeletePoolPosition: (data: MsgDeletePoolPosition) => EncodeObject;
    msgUpdatePoolSpotPrice: (data: MsgUpdatePoolSpotPrice) => EncodeObject;
    msgCreatePoolRanking: (data: MsgCreatePoolRanking) => EncodeObject;
    msgCreatePoolInfo: (data: MsgCreatePoolInfo) => EncodeObject;
    msgUpdatePoolInfo: (data: MsgUpdatePoolInfo) => EncodeObject;
    msgUpdatePoolPosition: (data: MsgUpdatePoolPosition) => EncodeObject;
}>;
interface QueryClientOptions {
    addr: string;
}
declare const queryClient: ({ addr: addr }?: QueryClientOptions) => Promise<Api<unknown>>;
export { txClient, queryClient, };
