import { Writer, Reader } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";
export declare const protobufPackage = "abag.quasarnode.qbank";
export interface Withdraw {
    id: number;
    riskProfile: string;
    vaultID: string;
    depositorAccAddress: string;
    /**
     * string amount = 5;
     * string denom = 6;
     */
    coin: Coin | undefined;
}
export declare const Withdraw: {
    encode(message: Withdraw, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): Withdraw;
    fromJSON(object: any): Withdraw;
    toJSON(message: Withdraw): unknown;
    fromPartial(object: DeepPartial<Withdraw>): Withdraw;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
