import { LockupTypes } from "../qbank/common";
import { Reader, Writer } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";
export declare const protobufPackage = "abag.quasarnode.qbank";
export interface MsgRequestDeposit {
    creator: string;
    riskProfile: string;
    vaultID: string;
    coin: Coin | undefined;
    /** string lockupPeriod = 5; */
    lockupPeriod: LockupTypes;
}
export interface MsgRequestDepositResponse {
}
export interface MsgRequestWithdraw {
    creator: string;
    riskProfile: string;
    vaultID: string;
    coin: Coin | undefined;
}
export interface MsgRequestWithdrawResponse {
}
export interface MsgRequestWithdrawAll {
    creator: string;
    /** string riskProfile = 2; */
    vaultID: string;
}
export interface MsgRequestWithdrawAllResponse {
}
export interface MsgClaimRewards {
    creator: string;
    vaultID: string;
}
export interface MsgClaimRewardsResponse {
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
export declare const MsgRequestWithdraw: {
    encode(message: MsgRequestWithdraw, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRequestWithdraw;
    fromJSON(object: any): MsgRequestWithdraw;
    toJSON(message: MsgRequestWithdraw): unknown;
    fromPartial(object: DeepPartial<MsgRequestWithdraw>): MsgRequestWithdraw;
};
export declare const MsgRequestWithdrawResponse: {
    encode(_: MsgRequestWithdrawResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRequestWithdrawResponse;
    fromJSON(_: any): MsgRequestWithdrawResponse;
    toJSON(_: MsgRequestWithdrawResponse): unknown;
    fromPartial(_: DeepPartial<MsgRequestWithdrawResponse>): MsgRequestWithdrawResponse;
};
export declare const MsgRequestWithdrawAll: {
    encode(message: MsgRequestWithdrawAll, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRequestWithdrawAll;
    fromJSON(object: any): MsgRequestWithdrawAll;
    toJSON(message: MsgRequestWithdrawAll): unknown;
    fromPartial(object: DeepPartial<MsgRequestWithdrawAll>): MsgRequestWithdrawAll;
};
export declare const MsgRequestWithdrawAllResponse: {
    encode(_: MsgRequestWithdrawAllResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgRequestWithdrawAllResponse;
    fromJSON(_: any): MsgRequestWithdrawAllResponse;
    toJSON(_: MsgRequestWithdrawAllResponse): unknown;
    fromPartial(_: DeepPartial<MsgRequestWithdrawAllResponse>): MsgRequestWithdrawAllResponse;
};
export declare const MsgClaimRewards: {
    encode(message: MsgClaimRewards, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgClaimRewards;
    fromJSON(object: any): MsgClaimRewards;
    toJSON(message: MsgClaimRewards): unknown;
    fromPartial(object: DeepPartial<MsgClaimRewards>): MsgClaimRewards;
};
export declare const MsgClaimRewardsResponse: {
    encode(_: MsgClaimRewardsResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgClaimRewardsResponse;
    fromJSON(_: any): MsgClaimRewardsResponse;
    toJSON(_: MsgClaimRewardsResponse): unknown;
    fromPartial(_: DeepPartial<MsgClaimRewardsResponse>): MsgClaimRewardsResponse;
};
/** Msg defines the Msg service. */
export interface Msg {
    RequestDeposit(request: MsgRequestDeposit): Promise<MsgRequestDepositResponse>;
    RequestWithdraw(request: MsgRequestWithdraw): Promise<MsgRequestWithdrawResponse>;
    /** this line is used by starport scaffolding # proto/tx/rpc */
    ClaimRewards(request: MsgClaimRewards): Promise<MsgClaimRewardsResponse>;
}
export declare class MsgClientImpl implements Msg {
    private readonly rpc;
    constructor(rpc: Rpc);
    RequestDeposit(request: MsgRequestDeposit): Promise<MsgRequestDepositResponse>;
    RequestWithdraw(request: MsgRequestWithdraw): Promise<MsgRequestWithdrawResponse>;
    ClaimRewards(request: MsgClaimRewards): Promise<MsgClaimRewardsResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
