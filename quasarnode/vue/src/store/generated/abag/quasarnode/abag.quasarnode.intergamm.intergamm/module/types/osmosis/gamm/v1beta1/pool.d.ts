import { Coin } from "../../../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "osmosis.gamm.v1beta1";
export interface PoolAsset {
    /**
     * Coins we are talking about,
     * the denomination must be unique amongst all PoolAssets for this pool.
     */
    token: Coin | undefined;
    /** Weight that is not normalized. This weight must be less than 2^50 */
    weight: string;
}
export declare const PoolAsset: {
    encode(message: PoolAsset, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): PoolAsset;
    fromJSON(object: any): PoolAsset;
    toJSON(message: PoolAsset): unknown;
    fromPartial(object: DeepPartial<PoolAsset>): PoolAsset;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
