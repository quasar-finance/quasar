import { StdFee } from "@cosmjs/launchpad";
import { Registry, OfflineSigner, EncodeObject } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgSendIbcWithdraw } from "./types/intergamm/tx";
import { MsgSendIbcJoinPool } from "./types/intergamm/tx";
import { MsgSendIbcExitPool } from "./types/intergamm/tx";
import { MsgRegisterAccount } from "./types/intergamm/tx";
import { MsgSendIbcCreatePool } from "./types/intergamm/tx";
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
    msgSendIbcWithdraw: (data: MsgSendIbcWithdraw) => EncodeObject;
    msgSendIbcJoinPool: (data: MsgSendIbcJoinPool) => EncodeObject;
    msgSendIbcExitPool: (data: MsgSendIbcExitPool) => EncodeObject;
    msgRegisterAccount: (data: MsgRegisterAccount) => EncodeObject;
    msgSendIbcCreatePool: (data: MsgSendIbcCreatePool) => EncodeObject;
}>;
interface QueryClientOptions {
    addr: string;
}
declare const queryClient: ({ addr: addr }?: QueryClientOptions) => Promise<Api<unknown>>;
export { txClient, queryClient, };
