import { Reader, Writer } from "protobufjs/minimal";
import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";
import { PageRequest, PageResponse } from "../cosmos/base/query/v1beta1/pagination";
import { PoolRanking } from "../qoracle/pool_ranking";
import { PoolSpotPrice } from "../qoracle/pool_spot_price";
import { PoolInfo } from "../qoracle/pool_info";
export declare const protobufPackage = "abag.quasarnode.qoracle";
/** QueryParamsRequest is request type for the Query/Params RPC method. */
export interface QueryParamsRequest {
}
/** QueryParamsResponse is response type for the Query/Params RPC method. */
export interface QueryParamsResponse {
    /** params holds all the parameters of this module. */
    params: Params | undefined;
}
export interface QueryGetPoolPositionRequest {
    poolId: string;
}
export interface QueryGetPoolPositionResponse {
    poolPosition: PoolPosition | undefined;
}
export interface QueryAllPoolPositionRequest {
    pagination: PageRequest | undefined;
}
export interface QueryAllPoolPositionResponse {
    poolPosition: PoolPosition[];
    pagination: PageResponse | undefined;
}
export interface QueryGetPoolRankingRequest {
}
export interface QueryGetPoolRankingResponse {
    PoolRanking: PoolRanking | undefined;
}
export interface QueryGetPoolSpotPriceRequest {
    poolId: string;
    denomIn: string;
    denomOut: string;
}
export interface QueryGetPoolSpotPriceResponse {
    poolSpotPrice: PoolSpotPrice | undefined;
}
export interface QueryAllPoolSpotPriceRequest {
    pagination: PageRequest | undefined;
}
export interface QueryAllPoolSpotPriceResponse {
    poolSpotPrice: PoolSpotPrice[];
    pagination: PageResponse | undefined;
}
export interface QueryGetPoolInfoRequest {
    poolId: string;
}
export interface QueryGetPoolInfoResponse {
    poolInfo: PoolInfo | undefined;
}
export interface QueryAllPoolInfoRequest {
    pagination: PageRequest | undefined;
}
export interface QueryAllPoolInfoResponse {
    poolInfo: PoolInfo[];
    pagination: PageResponse | undefined;
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
export declare const QueryGetPoolPositionRequest: {
    encode(message: QueryGetPoolPositionRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolPositionRequest;
    fromJSON(object: any): QueryGetPoolPositionRequest;
    toJSON(message: QueryGetPoolPositionRequest): unknown;
    fromPartial(object: DeepPartial<QueryGetPoolPositionRequest>): QueryGetPoolPositionRequest;
};
export declare const QueryGetPoolPositionResponse: {
    encode(message: QueryGetPoolPositionResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolPositionResponse;
    fromJSON(object: any): QueryGetPoolPositionResponse;
    toJSON(message: QueryGetPoolPositionResponse): unknown;
    fromPartial(object: DeepPartial<QueryGetPoolPositionResponse>): QueryGetPoolPositionResponse;
};
export declare const QueryAllPoolPositionRequest: {
    encode(message: QueryAllPoolPositionRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllPoolPositionRequest;
    fromJSON(object: any): QueryAllPoolPositionRequest;
    toJSON(message: QueryAllPoolPositionRequest): unknown;
    fromPartial(object: DeepPartial<QueryAllPoolPositionRequest>): QueryAllPoolPositionRequest;
};
export declare const QueryAllPoolPositionResponse: {
    encode(message: QueryAllPoolPositionResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllPoolPositionResponse;
    fromJSON(object: any): QueryAllPoolPositionResponse;
    toJSON(message: QueryAllPoolPositionResponse): unknown;
    fromPartial(object: DeepPartial<QueryAllPoolPositionResponse>): QueryAllPoolPositionResponse;
};
export declare const QueryGetPoolRankingRequest: {
    encode(_: QueryGetPoolRankingRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolRankingRequest;
    fromJSON(_: any): QueryGetPoolRankingRequest;
    toJSON(_: QueryGetPoolRankingRequest): unknown;
    fromPartial(_: DeepPartial<QueryGetPoolRankingRequest>): QueryGetPoolRankingRequest;
};
export declare const QueryGetPoolRankingResponse: {
    encode(message: QueryGetPoolRankingResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolRankingResponse;
    fromJSON(object: any): QueryGetPoolRankingResponse;
    toJSON(message: QueryGetPoolRankingResponse): unknown;
    fromPartial(object: DeepPartial<QueryGetPoolRankingResponse>): QueryGetPoolRankingResponse;
};
export declare const QueryGetPoolSpotPriceRequest: {
    encode(message: QueryGetPoolSpotPriceRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolSpotPriceRequest;
    fromJSON(object: any): QueryGetPoolSpotPriceRequest;
    toJSON(message: QueryGetPoolSpotPriceRequest): unknown;
    fromPartial(object: DeepPartial<QueryGetPoolSpotPriceRequest>): QueryGetPoolSpotPriceRequest;
};
export declare const QueryGetPoolSpotPriceResponse: {
    encode(message: QueryGetPoolSpotPriceResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolSpotPriceResponse;
    fromJSON(object: any): QueryGetPoolSpotPriceResponse;
    toJSON(message: QueryGetPoolSpotPriceResponse): unknown;
    fromPartial(object: DeepPartial<QueryGetPoolSpotPriceResponse>): QueryGetPoolSpotPriceResponse;
};
export declare const QueryAllPoolSpotPriceRequest: {
    encode(message: QueryAllPoolSpotPriceRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllPoolSpotPriceRequest;
    fromJSON(object: any): QueryAllPoolSpotPriceRequest;
    toJSON(message: QueryAllPoolSpotPriceRequest): unknown;
    fromPartial(object: DeepPartial<QueryAllPoolSpotPriceRequest>): QueryAllPoolSpotPriceRequest;
};
export declare const QueryAllPoolSpotPriceResponse: {
    encode(message: QueryAllPoolSpotPriceResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllPoolSpotPriceResponse;
    fromJSON(object: any): QueryAllPoolSpotPriceResponse;
    toJSON(message: QueryAllPoolSpotPriceResponse): unknown;
    fromPartial(object: DeepPartial<QueryAllPoolSpotPriceResponse>): QueryAllPoolSpotPriceResponse;
};
export declare const QueryGetPoolInfoRequest: {
    encode(message: QueryGetPoolInfoRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolInfoRequest;
    fromJSON(object: any): QueryGetPoolInfoRequest;
    toJSON(message: QueryGetPoolInfoRequest): unknown;
    fromPartial(object: DeepPartial<QueryGetPoolInfoRequest>): QueryGetPoolInfoRequest;
};
export declare const QueryGetPoolInfoResponse: {
    encode(message: QueryGetPoolInfoResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryGetPoolInfoResponse;
    fromJSON(object: any): QueryGetPoolInfoResponse;
    toJSON(message: QueryGetPoolInfoResponse): unknown;
    fromPartial(object: DeepPartial<QueryGetPoolInfoResponse>): QueryGetPoolInfoResponse;
};
export declare const QueryAllPoolInfoRequest: {
    encode(message: QueryAllPoolInfoRequest, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllPoolInfoRequest;
    fromJSON(object: any): QueryAllPoolInfoRequest;
    toJSON(message: QueryAllPoolInfoRequest): unknown;
    fromPartial(object: DeepPartial<QueryAllPoolInfoRequest>): QueryAllPoolInfoRequest;
};
export declare const QueryAllPoolInfoResponse: {
    encode(message: QueryAllPoolInfoResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): QueryAllPoolInfoResponse;
    fromJSON(object: any): QueryAllPoolInfoResponse;
    toJSON(message: QueryAllPoolInfoResponse): unknown;
    fromPartial(object: DeepPartial<QueryAllPoolInfoResponse>): QueryAllPoolInfoResponse;
};
/** Query defines the gRPC querier service. */
export interface Query {
    /** Parameters queries the parameters of the module. */
    Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
    /** Queries a PoolPosition by index. */
    PoolPosition(request: QueryGetPoolPositionRequest): Promise<QueryGetPoolPositionResponse>;
    /** Queries a list of PoolPosition items. */
    PoolPositionAll(request: QueryAllPoolPositionRequest): Promise<QueryAllPoolPositionResponse>;
    /** Queries a PoolRanking by index. */
    PoolRanking(request: QueryGetPoolRankingRequest): Promise<QueryGetPoolRankingResponse>;
    /** Queries a PoolSpotPrice by index. */
    PoolSpotPrice(request: QueryGetPoolSpotPriceRequest): Promise<QueryGetPoolSpotPriceResponse>;
    /** Queries a list of PoolSpotPrice items. */
    PoolSpotPriceAll(request: QueryAllPoolSpotPriceRequest): Promise<QueryAllPoolSpotPriceResponse>;
    /** Queries a PoolInfo by index. */
    PoolInfo(request: QueryGetPoolInfoRequest): Promise<QueryGetPoolInfoResponse>;
    /** Queries a list of PoolInfo items. */
    PoolInfoAll(request: QueryAllPoolInfoRequest): Promise<QueryAllPoolInfoResponse>;
}
export declare class QueryClientImpl implements Query {
    private readonly rpc;
    constructor(rpc: Rpc);
    Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
    PoolPosition(request: QueryGetPoolPositionRequest): Promise<QueryGetPoolPositionResponse>;
    PoolPositionAll(request: QueryAllPoolPositionRequest): Promise<QueryAllPoolPositionResponse>;
    PoolRanking(request: QueryGetPoolRankingRequest): Promise<QueryGetPoolRankingResponse>;
    PoolSpotPrice(request: QueryGetPoolSpotPriceRequest): Promise<QueryGetPoolSpotPriceResponse>;
    PoolSpotPriceAll(request: QueryAllPoolSpotPriceRequest): Promise<QueryAllPoolSpotPriceResponse>;
    PoolInfo(request: QueryGetPoolInfoRequest): Promise<QueryGetPoolInfoResponse>;
    PoolInfoAll(request: QueryAllPoolInfoRequest): Promise<QueryAllPoolInfoResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
