import { StdFee } from "@cosmjs/launchpad";
import { Registry, OfflineSigner, EncodeObject } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgRequestWithdraw } from "./types/qbank/tx";
import { MsgClaimRewards } from "./types/qbank/tx";
import { MsgRequestDeposit } from "./types/qbank/tx";
import { MsgRequestWithdrawAll } from "./types/qbank/tx";
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
    msgRequestWithdraw: (data: MsgRequestWithdraw) => EncodeObject;
    msgClaimRewards: (data: MsgClaimRewards) => EncodeObject;
    msgRequestDeposit: (data: MsgRequestDeposit) => EncodeObject;
    msgRequestWithdrawAll: (data: MsgRequestWithdrawAll) => EncodeObject;
}>;
interface QueryClientOptions {
    addr: string;
}
declare const queryClient: ({ addr: addr }?: QueryClientOptions) => Promise<Api<unknown>>;
export { txClient, queryClient, };
