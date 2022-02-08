import { Coin } from "../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qbank";
/** LockupTypes defines different types of locktypes to be used in the system for users deposit */
export declare enum LockupTypes {
    Invalid = 0,
    /** Days_7 - 7 Days */
    Days_7 = 1,
    /** Days_21 - 21 Days of lockup */
    Days_21 = 2,
    /** Months_1 - 1 Month of lockup */
    Months_1 = 3,
    /** Months_3 - 3 Months of lockup */
    Months_3 = 4,
    UNRECOGNIZED = -1
}
export declare function lockupTypesFromJSON(object: any): LockupTypes;
export declare function lockupTypesToJSON(object: LockupTypes): string;
/** QCoins defines encoding/decoding for the slice of sdk.coins to be used in KV stores. */
export interface QCoins {
    coins: Coin[];
}
/** QDenoms defines encoding/decoding for the slice of denoms to be used in KV stores */
export interface QDenoms {
    denoms: string[];
}
export declare const QCoins: {
    encode(message: QCoins, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QCoins;
    fromJSON(object: any): QCoins;
    toJSON(message: QCoins): unknown;
    fromPartial(object: DeepPartial<QCoins>): QCoins;
};
export declare const QDenoms: {
    encode(message: QDenoms, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QDenoms;
    fromJSON(object: any): QDenoms;
    toJSON(message: QDenoms): unknown;
    fromPartial(object: DeepPartial<QDenoms>): QDenoms;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
