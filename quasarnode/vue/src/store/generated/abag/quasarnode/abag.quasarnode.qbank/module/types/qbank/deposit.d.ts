import { LockupTypes } from "../qbank/common";
import { Writer, Reader } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";
export declare const protobufPackage = "abag.quasarnode.qbank";
/** Depsoit message object to be stored in the KV store. */
export interface Deposit {
    /** unique deposit id */
    id: number;
    /** Supported values are "LOW", "MID", "HIGH" */
    riskProfile: string;
    vaultID: string;
    depositorAccAddress: string;
    coin: Coin | undefined;
    /** string lockupPeriod = 6; // */
    lockupPeriod: LockupTypes;
}
export declare const Deposit: {
    encode(message: Deposit, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): Deposit;
    fromJSON(object: any): Deposit;
    toJSON(message: Deposit): unknown;
    fromPartial(object: DeepPartial<Deposit>): Deposit;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
