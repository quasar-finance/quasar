import { Coin } from "../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qbank";
export interface QCoins {
    coins: Coin[];
}
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
