import { Reader, Writer } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qbank";
export interface MsgRequestDeposit {
    creator: string;
    riskProfile: string;
    vaultID: string;
    amount: string;
    denom: string;
}
export interface MsgRequestDepositResponse {
}
export declare const MsgRequestDeposit: {
    encode(message: MsgRequestDeposit, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRequestDeposit;
    fromJSON(object: any): MsgRequestDeposit;
    toJSON(message: MsgRequestDeposit): unknown;
    fromPartial(object: DeepPartial<MsgRequestDeposit>): MsgRequestDeposit;
};
export declare const MsgRequestDepositResponse: {
    encode(_: MsgRequestDepositResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRequestDepositResponse;
    fromJSON(_: any): MsgRequestDepositResponse;
    toJSON(_: MsgRequestDepositResponse): unknown;
    fromPartial(_: DeepPartial<MsgRequestDepositResponse>): MsgRequestDepositResponse;
};
/** Msg defines the Msg service. */
export interface Msg {
    /** this line is used by starport scaffolding # proto/tx/rpc */
    RequestDeposit(request: MsgRequestDeposit): Promise<MsgRequestDepositResponse>;
}
export declare class MsgClientImpl implements Msg {
    private readonly rpc;
    constructor(rpc: Rpc);
    RequestDeposit(request: MsgRequestDeposit): Promise<MsgRequestDepositResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
