import { Writer, Reader } from "protobufjs/minimal";
import { BalancerPool } from "../osmosis/gamm/pool-models/balancer/balancerPool";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface PoolInfo {
    poolId: string;
    info: BalancerPool | undefined;
    lastUpdatedTime: number;
    creator: string;
}
export declare const PoolInfo: {
    encode(message: PoolInfo, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): PoolInfo;
    fromJSON(object: any): PoolInfo;
    toJSON(message: PoolInfo): unknown;
    fromPartial(object: DeepPartial<PoolInfo>): PoolInfo;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
