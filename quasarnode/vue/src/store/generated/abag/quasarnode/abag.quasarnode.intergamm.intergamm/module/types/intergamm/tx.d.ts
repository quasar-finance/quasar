import { Reader, Writer } from "protobufjs/minimal";
import { BalancerPoolParams } from "../osmosis/gamm/pool-models/balancer/balancerPool";
import { PoolAsset } from "../osmosis/gamm/v1beta1/pool";
import { Coin } from "../cosmos/base/v1beta1/coin";
export declare const protobufPackage = "abag.quasarnode.intergamm.intergamm";
export interface MsgSendIbcCreatePool {
    creator: string;
    port: string;
    channelID: string;
    timeoutTimestamp: number;
    poolParams: BalancerPoolParams | undefined;
    poolAssets: PoolAsset[];
    /**
     * repeated abag.quasarnode.osmosis.gamm.v1beta1.PoolAsset poolAssets = 6
     * [ (gogoproto.nullable) = false ];
     */
    futurePoolGovernor: string;
}
export interface MsgSendIbcCreatePoolResponse {
}
export interface MsgSendIbcJoinPool {
    creator: string;
    port: string;
    channelID: string;
    timeoutTimestamp: number;
    poolId: number;
    shareOutAmount: string;
    tokenInMaxs: Coin[];
}
export interface MsgSendIbcJoinPoolResponse {
}
export interface MsgSendIbcExitPool {
    creator: string;
    port: string;
    channelID: string;
    timeoutTimestamp: number;
    poolId: number;
    shareInAmount: string;
    tokenOutMins: Coin[];
}
export interface MsgSendIbcExitPoolResponse {
}
export interface MsgSendIbcWithdraw {
    creator: string;
    port: string;
    channelID: string;
    timeoutTimestamp: number;
    transferPort: string;
    transferChannel: string;
    receiver: string;
    assets: Coin[];
}
export interface MsgSendIbcWithdrawResponse {
}
export declare const MsgSendIbcCreatePool: {
    encode(message: MsgSendIbcCreatePool, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcCreatePool;
    fromJSON(object: any): MsgSendIbcCreatePool;
    toJSON(message: MsgSendIbcCreatePool): unknown;
    fromPartial(object: DeepPartial<MsgSendIbcCreatePool>): MsgSendIbcCreatePool;
};
export declare const MsgSendIbcCreatePoolResponse: {
    encode(_: MsgSendIbcCreatePoolResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcCreatePoolResponse;
    fromJSON(_: any): MsgSendIbcCreatePoolResponse;
    toJSON(_: MsgSendIbcCreatePoolResponse): unknown;
    fromPartial(_: DeepPartial<MsgSendIbcCreatePoolResponse>): MsgSendIbcCreatePoolResponse;
};
export declare const MsgSendIbcJoinPool: {
    encode(message: MsgSendIbcJoinPool, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcJoinPool;
    fromJSON(object: any): MsgSendIbcJoinPool;
    toJSON(message: MsgSendIbcJoinPool): unknown;
    fromPartial(object: DeepPartial<MsgSendIbcJoinPool>): MsgSendIbcJoinPool;
};
export declare const MsgSendIbcJoinPoolResponse: {
    encode(_: MsgSendIbcJoinPoolResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcJoinPoolResponse;
    fromJSON(_: any): MsgSendIbcJoinPoolResponse;
    toJSON(_: MsgSendIbcJoinPoolResponse): unknown;
    fromPartial(_: DeepPartial<MsgSendIbcJoinPoolResponse>): MsgSendIbcJoinPoolResponse;
};
export declare const MsgSendIbcExitPool: {
    encode(message: MsgSendIbcExitPool, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcExitPool;
    fromJSON(object: any): MsgSendIbcExitPool;
    toJSON(message: MsgSendIbcExitPool): unknown;
    fromPartial(object: DeepPartial<MsgSendIbcExitPool>): MsgSendIbcExitPool;
};
export declare const MsgSendIbcExitPoolResponse: {
    encode(_: MsgSendIbcExitPoolResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcExitPoolResponse;
    fromJSON(_: any): MsgSendIbcExitPoolResponse;
    toJSON(_: MsgSendIbcExitPoolResponse): unknown;
    fromPartial(_: DeepPartial<MsgSendIbcExitPoolResponse>): MsgSendIbcExitPoolResponse;
};
export declare const MsgSendIbcWithdraw: {
    encode(message: MsgSendIbcWithdraw, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcWithdraw;
    fromJSON(object: any): MsgSendIbcWithdraw;
    toJSON(message: MsgSendIbcWithdraw): unknown;
    fromPartial(object: DeepPartial<MsgSendIbcWithdraw>): MsgSendIbcWithdraw;
};
export declare const MsgSendIbcWithdrawResponse: {
    encode(_: MsgSendIbcWithdrawResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgSendIbcWithdrawResponse;
    fromJSON(_: any): MsgSendIbcWithdrawResponse;
    toJSON(_: MsgSendIbcWithdrawResponse): unknown;
    fromPartial(_: DeepPartial<MsgSendIbcWithdrawResponse>): MsgSendIbcWithdrawResponse;
};
/** Msg defines the Msg service. */
export interface Msg {
    SendIbcCreatePool(request: MsgSendIbcCreatePool): Promise<MsgSendIbcCreatePoolResponse>;
    SendIbcJoinPool(request: MsgSendIbcJoinPool): Promise<MsgSendIbcJoinPoolResponse>;
    SendIbcExitPool(request: MsgSendIbcExitPool): Promise<MsgSendIbcExitPoolResponse>;
    /** this line is used by starport scaffolding # proto/tx/rpc */
    SendIbcWithdraw(request: MsgSendIbcWithdraw): Promise<MsgSendIbcWithdrawResponse>;
}
export declare class MsgClientImpl implements Msg {
    private readonly rpc;
    constructor(rpc: Rpc);
    SendIbcCreatePool(request: MsgSendIbcCreatePool): Promise<MsgSendIbcCreatePoolResponse>;
    SendIbcJoinPool(request: MsgSendIbcJoinPool): Promise<MsgSendIbcJoinPoolResponse>;
    SendIbcExitPool(request: MsgSendIbcExitPool): Promise<MsgSendIbcExitPoolResponse>;
    SendIbcWithdraw(request: MsgSendIbcWithdraw): Promise<MsgSendIbcWithdrawResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
