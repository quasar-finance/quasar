import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface PoolSpotPrice {
    poolId: string;
    denomIn: string;
    denomOut: string;
    price: string;
    lastUpdatedTime: number;
    creator: string;
}
export declare const PoolSpotPrice: {
    encode(message: PoolSpotPrice, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): PoolSpotPrice;
    fromJSON(object: any): PoolSpotPrice;
    toJSON(message: PoolSpotPrice): unknown;
    fromPartial(object: DeepPartial<PoolSpotPrice>): PoolSpotPrice;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
