import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface PoolRanking {
    poolIdsSortedByAPY: string[];
    poolIdsSortedByTVL: string[];
    lastUpdatedTime: number;
    creator: string;
}
export declare const PoolRanking: {
    encode(message: PoolRanking, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): PoolRanking;
    fromJSON(object: any): PoolRanking;
    toJSON(message: PoolRanking): unknown;
    fromPartial(object: DeepPartial<PoolRanking>): PoolRanking;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
