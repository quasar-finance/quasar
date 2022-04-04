import { Reader, Writer } from "protobufjs/minimal";
import { Params } from "../qbank/params";
import { Deposit } from "../qbank/deposit";
import { PageRequest, PageResponse } from "../cosmos/base/query/v1beta1/pagination";
import { Withdraw } from "../qbank/withdraw";
import { FeeData } from "../qbank/fee_data";
import { QCoins } from "../qbank/common";
import { Coin } from "../cosmos/base/v1beta1/coin";
export declare const protobufPackage = "abag.quasarnode.qbank";
/** QueryParamsRequest is request type for the Query/Params RPC method. */
export interface QueryParamsRequest {
}
/** QueryParamsResponse is response type for the Query/Params RPC method. */
export interface QueryParamsResponse {
    /** params holds all the parameters of this module. */
    params: Params | undefined;
}
export interface QueryGetDepositRequest {
    id: number;
}
export interface QueryGetDepositResponse {
    Deposit: Deposit | undefined;
}
export interface QueryAllDepositRequest {
    pagination: PageRequest | undefined;
}
export interface QueryAllDepositResponse {
    Deposit: Deposit[];
    pagination: PageResponse | undefined;
}
export interface QueryUserDenomDepositRequest {
    userAcc: string;
    denom: string;
}
export interface QueryUserDenomDepositResponse {
    amount: number;
}
export interface QueryGetWithdrawRequest {
    id: number;
}
export interface QueryGetWithdrawResponse {
    Withdraw: Withdraw | undefined;
}
export interface QueryAllWithdrawRequest {
    pagination: PageRequest | undefined;
}
export interface QueryAllWithdrawResponse {
    Withdraw: Withdraw[];
    pagination: PageResponse | undefined;
}
export interface QueryGetFeeDataRequest {
}
export interface QueryGetFeeDataResponse {
    FeeData: FeeData | undefined;
}
export interface QueryUserDepositRequest {
    userAcc: string;
}
export interface QueryUserDepositResponse {
    coins: QCoins | undefined;
}
export interface QueryUserDenomLockupDepositRequest {
    userAcc: string;
    denom: string;
    lockupType: string;
}
export interface QueryUserDenomLockupDepositResponse {
    amount: number;
}
export interface QueryUserDenomEpochLockupDepositRequest {
    userAcc: string;
    denom: string;
    epochDay: number[];
    lockupType: string;
}
export interface QueryUserDenomEpochLockupDepositResponse {
    amount: number;
}
export interface QueryUserWithdrawRequest {
    userAcc: string;
}
export interface QueryUserWithdrawResponse {
    coins: QCoins | undefined;
}
export interface QueryUserDenomWithdrawRequest {
    userAcc: string;
    denom: string;
}
export interface QueryUserDenomWithdrawResponse {
    amount: number;
}
export interface QueryUserClaimRewardsRequest {
    userAcc: string;
}
export interface QueryUserClaimRewardsResponse {
    coins: QCoins | undefined;
}
export interface QueryWithdrableRequest {
    userAccount: string;
    denom: string;
}
export interface QueryWithdrableResponse {
    coin: Coin | undefined;
}
export declare const QueryParamsRequest: {
    encode(_: QueryParamsRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryParamsRequest;
    fromJSON(_: any): QueryParamsRequest;
    toJSON(_: QueryParamsRequest): unknown;
    fromPartial(_: DeepPartial<QueryParamsRequest>): QueryParamsRequest;
};
export declare const QueryParamsResponse: {
    encode(message: QueryParamsResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryParamsResponse;
    fromJSON(object: any): QueryParamsResponse;
    toJSON(message: QueryParamsResponse): unknown;
    fromPartial(object: DeepPartial<QueryParamsResponse>): QueryParamsResponse;
};
export declare const QueryGetDepositRequest: {
    encode(message: QueryGetDepositRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetDepositRequest;
    fromJSON(object: any): QueryGetDepositRequest;
    toJSON(message: QueryGetDepositRequest): unknown;
    fromPartial(object: DeepPartial<QueryGetDepositRequest>): QueryGetDepositRequest;
};
export declare const QueryGetDepositResponse: {
    encode(message: QueryGetDepositResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetDepositResponse;
    fromJSON(object: any): QueryGetDepositResponse;
    toJSON(message: QueryGetDepositResponse): unknown;
    fromPartial(object: DeepPartial<QueryGetDepositResponse>): QueryGetDepositResponse;
};
export declare const QueryAllDepositRequest: {
    encode(message: QueryAllDepositRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllDepositRequest;
    fromJSON(object: any): QueryAllDepositRequest;
    toJSON(message: QueryAllDepositRequest): unknown;
    fromPartial(object: DeepPartial<QueryAllDepositRequest>): QueryAllDepositRequest;
};
export declare const QueryAllDepositResponse: {
    encode(message: QueryAllDepositResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllDepositResponse;
    fromJSON(object: any): QueryAllDepositResponse;
    toJSON(message: QueryAllDepositResponse): unknown;
    fromPartial(object: DeepPartial<QueryAllDepositResponse>): QueryAllDepositResponse;
};
export declare const QueryUserDenomDepositRequest: {
    encode(message: QueryUserDenomDepositRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomDepositRequest;
    fromJSON(object: any): QueryUserDenomDepositRequest;
    toJSON(message: QueryUserDenomDepositRequest): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomDepositRequest>): QueryUserDenomDepositRequest;
};
export declare const QueryUserDenomDepositResponse: {
    encode(message: QueryUserDenomDepositResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomDepositResponse;
    fromJSON(object: any): QueryUserDenomDepositResponse;
    toJSON(message: QueryUserDenomDepositResponse): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomDepositResponse>): QueryUserDenomDepositResponse;
};
export declare const QueryGetWithdrawRequest: {
    encode(message: QueryGetWithdrawRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetWithdrawRequest;
    fromJSON(object: any): QueryGetWithdrawRequest;
    toJSON(message: QueryGetWithdrawRequest): unknown;
    fromPartial(object: DeepPartial<QueryGetWithdrawRequest>): QueryGetWithdrawRequest;
};
export declare const QueryGetWithdrawResponse: {
    encode(message: QueryGetWithdrawResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetWithdrawResponse;
    fromJSON(object: any): QueryGetWithdrawResponse;
    toJSON(message: QueryGetWithdrawResponse): unknown;
    fromPartial(object: DeepPartial<QueryGetWithdrawResponse>): QueryGetWithdrawResponse;
};
export declare const QueryAllWithdrawRequest: {
    encode(message: QueryAllWithdrawRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllWithdrawRequest;
    fromJSON(object: any): QueryAllWithdrawRequest;
    toJSON(message: QueryAllWithdrawRequest): unknown;
    fromPartial(object: DeepPartial<QueryAllWithdrawRequest>): QueryAllWithdrawRequest;
};
export declare const QueryAllWithdrawResponse: {
    encode(message: QueryAllWithdrawResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllWithdrawResponse;
    fromJSON(object: any): QueryAllWithdrawResponse;
    toJSON(message: QueryAllWithdrawResponse): unknown;
    fromPartial(object: DeepPartial<QueryAllWithdrawResponse>): QueryAllWithdrawResponse;
};
export declare const QueryGetFeeDataRequest: {
    encode(_: QueryGetFeeDataRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetFeeDataRequest;
    fromJSON(_: any): QueryGetFeeDataRequest;
    toJSON(_: QueryGetFeeDataRequest): unknown;
    fromPartial(_: DeepPartial<QueryGetFeeDataRequest>): QueryGetFeeDataRequest;
};
export declare const QueryGetFeeDataResponse: {
    encode(message: QueryGetFeeDataResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetFeeDataResponse;
    fromJSON(object: any): QueryGetFeeDataResponse;
    toJSON(message: QueryGetFeeDataResponse): unknown;
    fromPartial(object: DeepPartial<QueryGetFeeDataResponse>): QueryGetFeeDataResponse;
};
export declare const QueryUserDepositRequest: {
    encode(message: QueryUserDepositRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDepositRequest;
    fromJSON(object: any): QueryUserDepositRequest;
    toJSON(message: QueryUserDepositRequest): unknown;
    fromPartial(object: DeepPartial<QueryUserDepositRequest>): QueryUserDepositRequest;
};
export declare const QueryUserDepositResponse: {
    encode(message: QueryUserDepositResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDepositResponse;
    fromJSON(object: any): QueryUserDepositResponse;
    toJSON(message: QueryUserDepositResponse): unknown;
    fromPartial(object: DeepPartial<QueryUserDepositResponse>): QueryUserDepositResponse;
};
export declare const QueryUserDenomLockupDepositRequest: {
    encode(message: QueryUserDenomLockupDepositRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomLockupDepositRequest;
    fromJSON(object: any): QueryUserDenomLockupDepositRequest;
    toJSON(message: QueryUserDenomLockupDepositRequest): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomLockupDepositRequest>): QueryUserDenomLockupDepositRequest;
};
export declare const QueryUserDenomLockupDepositResponse: {
    encode(message: QueryUserDenomLockupDepositResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomLockupDepositResponse;
    fromJSON(object: any): QueryUserDenomLockupDepositResponse;
    toJSON(message: QueryUserDenomLockupDepositResponse): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomLockupDepositResponse>): QueryUserDenomLockupDepositResponse;
};
export declare const QueryUserDenomEpochLockupDepositRequest: {
    encode(message: QueryUserDenomEpochLockupDepositRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomEpochLockupDepositRequest;
    fromJSON(object: any): QueryUserDenomEpochLockupDepositRequest;
    toJSON(message: QueryUserDenomEpochLockupDepositRequest): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomEpochLockupDepositRequest>): QueryUserDenomEpochLockupDepositRequest;
};
export declare const QueryUserDenomEpochLockupDepositResponse: {
    encode(message: QueryUserDenomEpochLockupDepositResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomEpochLockupDepositResponse;
    fromJSON(object: any): QueryUserDenomEpochLockupDepositResponse;
    toJSON(message: QueryUserDenomEpochLockupDepositResponse): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomEpochLockupDepositResponse>): QueryUserDenomEpochLockupDepositResponse;
};
export declare const QueryUserWithdrawRequest: {
    encode(message: QueryUserWithdrawRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserWithdrawRequest;
    fromJSON(object: any): QueryUserWithdrawRequest;
    toJSON(message: QueryUserWithdrawRequest): unknown;
    fromPartial(object: DeepPartial<QueryUserWithdrawRequest>): QueryUserWithdrawRequest;
};
export declare const QueryUserWithdrawResponse: {
    encode(message: QueryUserWithdrawResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserWithdrawResponse;
    fromJSON(object: any): QueryUserWithdrawResponse;
    toJSON(message: QueryUserWithdrawResponse): unknown;
    fromPartial(object: DeepPartial<QueryUserWithdrawResponse>): QueryUserWithdrawResponse;
};
export declare const QueryUserDenomWithdrawRequest: {
    encode(message: QueryUserDenomWithdrawRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomWithdrawRequest;
    fromJSON(object: any): QueryUserDenomWithdrawRequest;
    toJSON(message: QueryUserDenomWithdrawRequest): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomWithdrawRequest>): QueryUserDenomWithdrawRequest;
};
export declare const QueryUserDenomWithdrawResponse: {
    encode(message: QueryUserDenomWithdrawResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserDenomWithdrawResponse;
    fromJSON(object: any): QueryUserDenomWithdrawResponse;
    toJSON(message: QueryUserDenomWithdrawResponse): unknown;
    fromPartial(object: DeepPartial<QueryUserDenomWithdrawResponse>): QueryUserDenomWithdrawResponse;
};
export declare const QueryUserClaimRewardsRequest: {
    encode(message: QueryUserClaimRewardsRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserClaimRewardsRequest;
    fromJSON(object: any): QueryUserClaimRewardsRequest;
    toJSON(message: QueryUserClaimRewardsRequest): unknown;
    fromPartial(object: DeepPartial<QueryUserClaimRewardsRequest>): QueryUserClaimRewardsRequest;
};
export declare const QueryUserClaimRewardsResponse: {
    encode(message: QueryUserClaimRewardsResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryUserClaimRewardsResponse;
    fromJSON(object: any): QueryUserClaimRewardsResponse;
    toJSON(message: QueryUserClaimRewardsResponse): unknown;
    fromPartial(object: DeepPartial<QueryUserClaimRewardsResponse>): QueryUserClaimRewardsResponse;
};
export declare const QueryWithdrableRequest: {
    encode(message: QueryWithdrableRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryWithdrableRequest;
    fromJSON(object: any): QueryWithdrableRequest;
    toJSON(message: QueryWithdrableRequest): unknown;
    fromPartial(object: DeepPartial<QueryWithdrableRequest>): QueryWithdrableRequest;
};
export declare const QueryWithdrableResponse: {
    encode(message: QueryWithdrableResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryWithdrableResponse;
    fromJSON(object: any): QueryWithdrableResponse;
    toJSON(message: QueryWithdrableResponse): unknown;
    fromPartial(object: DeepPartial<QueryWithdrableResponse>): QueryWithdrableResponse;
};
/** Query defines the gRPC querier service. */
export interface Query {
    /** Parameters queries the parameters of the module. */
    Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
    /** Queries a Deposit by id. */
    Deposit(request: QueryGetDepositRequest): Promise<QueryGetDepositResponse>;
    /** Queries a list of Deposit items. */
    DepositAll(request: QueryAllDepositRequest): Promise<QueryAllDepositResponse>;
    /** Queries a list of UserDenomDeposit items. */
    UserDenomDeposit(request: QueryUserDenomDepositRequest): Promise<QueryUserDenomDepositResponse>;
    /** Queries a Withdraw by id. */
    Withdraw(request: QueryGetWithdrawRequest): Promise<QueryGetWithdrawResponse>;
    /** Queries a list of Withdraw items. */
    WithdrawAll(request: QueryAllWithdrawRequest): Promise<QueryAllWithdrawResponse>;
    /** Queries a FeeData by index. */
    FeeData(request: QueryGetFeeDataRequest): Promise<QueryGetFeeDataResponse>;
    /** Queries a list of UserDeposit items. */
    UserDeposit(request: QueryUserDepositRequest): Promise<QueryUserDepositResponse>;
    /** Queries a list of UserDenomLockupDeposit items. */
    UserDenomLockupDeposit(request: QueryUserDenomLockupDepositRequest): Promise<QueryUserDenomLockupDepositResponse>;
    /** Queries a list of UserDenomEpochLockupDeposit items. */
    UserDenomEpochLockupDeposit(request: QueryUserDenomEpochLockupDepositRequest): Promise<QueryUserDenomEpochLockupDepositResponse>;
    /** Queries a list of UserWithdraw items. */
    UserWithdraw(request: QueryUserWithdrawRequest): Promise<QueryUserWithdrawResponse>;
    /** Queries a list of UserDenomWithdraw items. */
    UserDenomWithdraw(request: QueryUserDenomWithdrawRequest): Promise<QueryUserDenomWithdrawResponse>;
    /** Queries a list of UserClaimRewards items. */
    UserClaimRewards(request: QueryUserClaimRewardsRequest): Promise<QueryUserClaimRewardsResponse>;
    /** Queries a list of Withdrable items. */
    Withdrable(request: QueryWithdrableRequest): Promise<QueryWithdrableResponse>;
}
export declare class QueryClientImpl implements Query {
    private readonly rpc;
    constructor(rpc: Rpc);
    Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
    Deposit(request: QueryGetDepositRequest): Promise<QueryGetDepositResponse>;
    DepositAll(request: QueryAllDepositRequest): Promise<QueryAllDepositResponse>;
    UserDenomDeposit(request: QueryUserDenomDepositRequest): Promise<QueryUserDenomDepositResponse>;
    Withdraw(request: QueryGetWithdrawRequest): Promise<QueryGetWithdrawResponse>;
    WithdrawAll(request: QueryAllWithdrawRequest): Promise<QueryAllWithdrawResponse>;
    FeeData(request: QueryGetFeeDataRequest): Promise<QueryGetFeeDataResponse>;
    UserDeposit(request: QueryUserDepositRequest): Promise<QueryUserDepositResponse>;
    UserDenomLockupDeposit(request: QueryUserDenomLockupDepositRequest): Promise<QueryUserDenomLockupDepositResponse>;
    UserDenomEpochLockupDeposit(request: QueryUserDenomEpochLockupDepositRequest): Promise<QueryUserDenomEpochLockupDepositResponse>;
    UserWithdraw(request: QueryUserWithdrawRequest): Promise<QueryUserWithdrawResponse>;
    UserDenomWithdraw(request: QueryUserDenomWithdrawRequest): Promise<QueryUserDenomWithdrawResponse>;
    UserClaimRewards(request: QueryUserClaimRewardsRequest): Promise<QueryUserClaimRewardsResponse>;
    Withdrable(request: QueryWithdrableRequest): Promise<QueryWithdrableResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
