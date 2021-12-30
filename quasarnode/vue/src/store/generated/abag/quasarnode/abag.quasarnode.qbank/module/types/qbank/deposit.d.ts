import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qbank";
export interface Deposit {
    id: number;
    riskProfile: string;
    vaultID: string;
    depositorAccAddress: string;
    amount: string;
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
