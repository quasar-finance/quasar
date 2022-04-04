import { Reader, Writer } from "protobufjs/minimal";
import { BalancerPoolParams } from "../osmosis/gamm/pool-models/balancer/balancerPool";
import { PoolAsset } from "../osmosis/gamm/v1beta1/pool";
import { Coin } from "../cosmos/base/v1beta1/coin";
export declare const protobufPackage = "abag.quasarnode.intergamm";
export interface MsgRegisterAccount {
    creator: string;
    connectionId: string;
}
export interface MsgRegisterAccountResponse {
}
export interface MsgCreatePool {
    creator: string;
    connectionId: string;
    timeoutTimestamp: number;
    poolParams: BalancerPoolParams | undefined;
    poolAssets: PoolAsset[];
    futurePoolGovernor: string;
}
export interface MsgCreatePoolResponse {
}
export interface MsgJoinPool {
    creator: string;
    connectionId: string;
    timeoutTimestamp: number;
    poolId: number;
    shareOutAmount: string;
    tokenInMaxs: Coin[];
}
export interface MsgJoinPoolResponse {
}
export interface MsgExitPool {
    creator: string;
    connectionId: string;
    timeoutTimestamp: number;
    poolId: number;
    shareInAmount: string;
    tokenOutMins: Coin[];
}
export interface MsgExitPoolResponse {
}
export interface MsgIbcTransfer {
    creator: string;
}
export interface MsgIbcTransferResponse {
}
export declare const MsgRegisterAccount: {
    encode(message: MsgRegisterAccount, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRegisterAccount;
    fromJSON(object: any): MsgRegisterAccount;
    toJSON(message: MsgRegisterAccount): unknown;
    fromPartial(object: DeepPartial<MsgRegisterAccount>): MsgRegisterAccount;
};
export declare const MsgRegisterAccountResponse: {
    encode(_: MsgRegisterAccountResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRegisterAccountResponse;
    fromJSON(_: any): MsgRegisterAccountResponse;
    toJSON(_: MsgRegisterAccountResponse): unknown;
    fromPartial(_: DeepPartial<MsgRegisterAccountResponse>): MsgRegisterAccountResponse;
};
export declare const MsgCreatePool: {
    encode(message: MsgCreatePool, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePool;
    fromJSON(object: any): MsgCreatePool;
    toJSON(message: MsgCreatePool): unknown;
    fromPartial(object: DeepPartial<MsgCreatePool>): MsgCreatePool;
};
export declare const MsgCreatePoolResponse: {
    encode(_: MsgCreatePoolResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolResponse;
    fromJSON(_: any): MsgCreatePoolResponse;
    toJSON(_: MsgCreatePoolResponse): unknown;
    fromPartial(_: DeepPartial<MsgCreatePoolResponse>): MsgCreatePoolResponse;
};
export declare const MsgJoinPool: {
    encode(message: MsgJoinPool, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgJoinPool;
    fromJSON(object: any): MsgJoinPool;
    toJSON(message: MsgJoinPool): unknown;
    fromPartial(object: DeepPartial<MsgJoinPool>): MsgJoinPool;
};
export declare const MsgJoinPoolResponse: {
    encode(_: MsgJoinPoolResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgJoinPoolResponse;
    fromJSON(_: any): MsgJoinPoolResponse;
    toJSON(_: MsgJoinPoolResponse): unknown;
    fromPartial(_: DeepPartial<MsgJoinPoolResponse>): MsgJoinPoolResponse;
};
export declare const MsgExitPool: {
    encode(message: MsgExitPool, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgExitPool;
    fromJSON(object: any): MsgExitPool;
    toJSON(message: MsgExitPool): unknown;
    fromPartial(object: DeepPartial<MsgExitPool>): MsgExitPool;
};
export declare const MsgExitPoolResponse: {
    encode(_: MsgExitPoolResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgExitPoolResponse;
    fromJSON(_: any): MsgExitPoolResponse;
    toJSON(_: MsgExitPoolResponse): unknown;
    fromPartial(_: DeepPartial<MsgExitPoolResponse>): MsgExitPoolResponse;
};
export declare const MsgIbcTransfer: {
    encode(message: MsgIbcTransfer, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgIbcTransfer;
    fromJSON(object: any): MsgIbcTransfer;
    toJSON(message: MsgIbcTransfer): unknown;
    fromPartial(object: DeepPartial<MsgIbcTransfer>): MsgIbcTransfer;
};
export declare const MsgIbcTransferResponse: {
    encode(_: MsgIbcTransferResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgIbcTransferResponse;
    fromJSON(_: any): MsgIbcTransferResponse;
    toJSON(_: MsgIbcTransferResponse): unknown;
    fromPartial(_: DeepPartial<MsgIbcTransferResponse>): MsgIbcTransferResponse;
};
/** Msg defines the Msg service. */
export interface Msg {
    RegisterAccount(request: MsgRegisterAccount): Promise<MsgRegisterAccountResponse>;
    CreatePool(request: MsgCreatePool): Promise<MsgCreatePoolResponse>;
    JoinPool(request: MsgJoinPool): Promise<MsgJoinPoolResponse>;
    ExitPool(request: MsgExitPool): Promise<MsgExitPoolResponse>;
    /** this line is used by starport scaffolding # proto/tx/rpc */
    IbcTransfer(request: MsgIbcTransfer): Promise<MsgIbcTransferResponse>;
}
export declare class MsgClientImpl implements Msg {
    private readonly rpc;
    constructor(rpc: Rpc);
    RegisterAccount(request: MsgRegisterAccount): Promise<MsgRegisterAccountResponse>;
    CreatePool(request: MsgCreatePool): Promise<MsgCreatePoolResponse>;
    JoinPool(request: MsgJoinPool): Promise<MsgJoinPoolResponse>;
    ExitPool(request: MsgExitPool): Promise<MsgExitPoolResponse>;
    IbcTransfer(request: MsgIbcTransfer): Promise<MsgIbcTransferResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
