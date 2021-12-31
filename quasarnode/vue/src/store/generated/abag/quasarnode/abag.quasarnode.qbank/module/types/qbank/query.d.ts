import { Reader, Writer } from "protobufjs/minimal";
import { Params } from "../qbank/params";
import { Deposit } from "../qbank/deposit";
import { PageRequest, PageResponse } from "../cosmos/base/query/v1beta1/pagination";
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
}
export declare class QueryClientImpl implements Query {
    private readonly rpc;
    constructor(rpc: Rpc);
    Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
    Deposit(request: QueryGetDepositRequest): Promise<QueryGetDepositResponse>;
    DepositAll(request: QueryAllDepositRequest): Promise<QueryAllDepositResponse>;
    UserDenomDeposit(request: QueryUserDenomDepositRequest): Promise<QueryUserDenomDepositResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
