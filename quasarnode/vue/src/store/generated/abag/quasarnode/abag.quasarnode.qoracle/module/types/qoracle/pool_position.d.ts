import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface PoolPosition {
    aPY: number;
    tVL: number;
    lastUpdatedTime: number;
    creator: string;
}
export declare const PoolPosition: {
    encode(message: PoolPosition, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): PoolPosition;
    fromJSON(object: any): PoolPosition;
    toJSON(message: PoolPosition): unknown;
    fromPartial(object: DeepPartial<PoolPosition>): PoolPosition;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
