import { Reader, Writer } from "protobufjs/minimal";
import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";
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
    PoolID: number;
}
export interface QueryGetPoolPositionResponse {
    PoolPosition: PoolPosition | undefined;
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
/** Query defines the gRPC querier service. */
export interface Query {
    /** Parameters queries the parameters of the module. */
    Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
    /** Queries a PoolPosition by index. */
    PoolPosition(request: QueryGetPoolPositionRequest): Promise<QueryGetPoolPositionResponse>;
}
export declare class QueryClientImpl implements Query {
    private readonly rpc;
    constructor(rpc: Rpc);
    Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
    PoolPosition(request: QueryGetPoolPositionRequest): Promise<QueryGetPoolPositionResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
