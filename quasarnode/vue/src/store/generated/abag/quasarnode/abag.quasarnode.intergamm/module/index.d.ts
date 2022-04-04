import { StdFee } from "@cosmjs/launchpad";
import { Registry, OfflineSigner, EncodeObject } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgJoinPool } from "./types/intergamm/tx";
import { MsgRegisterAccount } from "./types/intergamm/tx";
import { MsgIbcTransfer } from "./types/intergamm/tx";
import { MsgCreatePool } from "./types/intergamm/tx";
import { MsgExitPool } from "./types/intergamm/tx";
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
    msgJoinPool: (data: MsgJoinPool) => EncodeObject;
    msgRegisterAccount: (data: MsgRegisterAccount) => EncodeObject;
    msgIbcTransfer: (data: MsgIbcTransfer) => EncodeObject;
    msgCreatePool: (data: MsgCreatePool) => EncodeObject;
    msgExitPool: (data: MsgExitPool) => EncodeObject;
}>;
interface QueryClientOptions {
    addr: string;
}
declare const queryClient: ({ addr: addr }?: QueryClientOptions) => Promise<Api<unknown>>;
export { txClient, queryClient, };
