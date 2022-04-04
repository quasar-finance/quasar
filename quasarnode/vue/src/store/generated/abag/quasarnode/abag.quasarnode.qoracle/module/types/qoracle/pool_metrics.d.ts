import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface PoolMetrics {
    aPY: string;
    tVL: string;
}
export declare const PoolMetrics: {
    encode(message: PoolMetrics, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): PoolMetrics;
    fromJSON(object: any): PoolMetrics;
    toJSON(message: PoolMetrics): unknown;
    fromPartial(object: DeepPartial<PoolMetrics>): PoolMetrics;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
