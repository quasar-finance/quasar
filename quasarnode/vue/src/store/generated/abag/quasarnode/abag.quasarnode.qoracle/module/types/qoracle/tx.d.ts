import { Reader, Writer } from "protobufjs/minimal";
import { PoolMetrics } from "../qoracle/pool_metrics";
import { BalancerPool } from "../osmosis/gamm/pool-models/balancer/balancerPool";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface MsgCreatePoolPosition {
    creator: string;
    poolId: string;
    metrics: PoolMetrics | undefined;
    lastUpdatedTime: number;
}
export interface MsgCreatePoolPositionResponse {
}
export interface MsgUpdatePoolPosition {
    creator: string;
    poolId: string;
    metrics: PoolMetrics | undefined;
    lastUpdatedTime: number;
}
export interface MsgUpdatePoolPositionResponse {
}
export interface MsgDeletePoolPosition {
    creator: string;
    poolId: string;
}
export interface MsgDeletePoolPositionResponse {
}
export interface MsgCreatePoolRanking {
    creator: string;
    poolIdsSortedByAPY: string[];
    poolIdsSortedByTVL: string[];
    lastUpdatedTime: number;
}
export interface MsgCreatePoolRankingResponse {
}
export interface MsgUpdatePoolRanking {
    creator: string;
    poolIdsSortedByAPY: string[];
    poolIdsSortedByTVL: string[];
    lastUpdatedTime: number;
}
export interface MsgUpdatePoolRankingResponse {
}
export interface MsgDeletePoolRanking {
    creator: string;
}
export interface MsgDeletePoolRankingResponse {
}
export interface MsgCreatePoolSpotPrice {
    creator: string;
    poolId: string;
    denomIn: string;
    denomOut: string;
    price: string;
    lastUpdatedTime: number;
}
export interface MsgCreatePoolSpotPriceResponse {
}
export interface MsgUpdatePoolSpotPrice {
    creator: string;
    poolId: string;
    denomIn: string;
    denomOut: string;
    price: string;
    lastUpdatedTime: number;
}
export interface MsgUpdatePoolSpotPriceResponse {
}
export interface MsgDeletePoolSpotPrice {
    creator: string;
    poolId: string;
    denomIn: string;
    denomOut: string;
}
export interface MsgDeletePoolSpotPriceResponse {
}
export interface MsgCreatePoolInfo {
    creator: string;
    poolId: string;
    info: BalancerPool | undefined;
    lastUpdatedTime: number;
}
export interface MsgCreatePoolInfoResponse {
}
export interface MsgUpdatePoolInfo {
    creator: string;
    poolId: string;
    info: BalancerPool | undefined;
    lastUpdatedTime: number;
}
export interface MsgUpdatePoolInfoResponse {
}
export interface MsgDeletePoolInfo {
    creator: string;
    poolId: string;
}
export interface MsgDeletePoolInfoResponse {
}
export declare const MsgCreatePoolPosition: {
    encode(message: MsgCreatePoolPosition, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolPosition;
    fromJSON(object: any): MsgCreatePoolPosition;
    toJSON(message: MsgCreatePoolPosition): unknown;
    fromPartial(object: DeepPartial<MsgCreatePoolPosition>): MsgCreatePoolPosition;
};
export declare const MsgCreatePoolPositionResponse: {
    encode(_: MsgCreatePoolPositionResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolPositionResponse;
    fromJSON(_: any): MsgCreatePoolPositionResponse;
    toJSON(_: MsgCreatePoolPositionResponse): unknown;
    fromPartial(_: DeepPartial<MsgCreatePoolPositionResponse>): MsgCreatePoolPositionResponse;
};
export declare const MsgUpdatePoolPosition: {
    encode(message: MsgUpdatePoolPosition, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolPosition;
    fromJSON(object: any): MsgUpdatePoolPosition;
    toJSON(message: MsgUpdatePoolPosition): unknown;
    fromPartial(object: DeepPartial<MsgUpdatePoolPosition>): MsgUpdatePoolPosition;
};
export declare const MsgUpdatePoolPositionResponse: {
    encode(_: MsgUpdatePoolPositionResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolPositionResponse;
    fromJSON(_: any): MsgUpdatePoolPositionResponse;
    toJSON(_: MsgUpdatePoolPositionResponse): unknown;
    fromPartial(_: DeepPartial<MsgUpdatePoolPositionResponse>): MsgUpdatePoolPositionResponse;
};
export declare const MsgDeletePoolPosition: {
    encode(message: MsgDeletePoolPosition, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolPosition;
    fromJSON(object: any): MsgDeletePoolPosition;
    toJSON(message: MsgDeletePoolPosition): unknown;
    fromPartial(object: DeepPartial<MsgDeletePoolPosition>): MsgDeletePoolPosition;
};
export declare const MsgDeletePoolPositionResponse: {
    encode(_: MsgDeletePoolPositionResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolPositionResponse;
    fromJSON(_: any): MsgDeletePoolPositionResponse;
    toJSON(_: MsgDeletePoolPositionResponse): unknown;
    fromPartial(_: DeepPartial<MsgDeletePoolPositionResponse>): MsgDeletePoolPositionResponse;
};
export declare const MsgCreatePoolRanking: {
    encode(message: MsgCreatePoolRanking, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolRanking;
    fromJSON(object: any): MsgCreatePoolRanking;
    toJSON(message: MsgCreatePoolRanking): unknown;
    fromPartial(object: DeepPartial<MsgCreatePoolRanking>): MsgCreatePoolRanking;
};
export declare const MsgCreatePoolRankingResponse: {
    encode(_: MsgCreatePoolRankingResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolRankingResponse;
    fromJSON(_: any): MsgCreatePoolRankingResponse;
    toJSON(_: MsgCreatePoolRankingResponse): unknown;
    fromPartial(_: DeepPartial<MsgCreatePoolRankingResponse>): MsgCreatePoolRankingResponse;
};
export declare const MsgUpdatePoolRanking: {
    encode(message: MsgUpdatePoolRanking, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolRanking;
    fromJSON(object: any): MsgUpdatePoolRanking;
    toJSON(message: MsgUpdatePoolRanking): unknown;
    fromPartial(object: DeepPartial<MsgUpdatePoolRanking>): MsgUpdatePoolRanking;
};
export declare const MsgUpdatePoolRankingResponse: {
    encode(_: MsgUpdatePoolRankingResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolRankingResponse;
    fromJSON(_: any): MsgUpdatePoolRankingResponse;
    toJSON(_: MsgUpdatePoolRankingResponse): unknown;
    fromPartial(_: DeepPartial<MsgUpdatePoolRankingResponse>): MsgUpdatePoolRankingResponse;
};
export declare const MsgDeletePoolRanking: {
    encode(message: MsgDeletePoolRanking, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolRanking;
    fromJSON(object: any): MsgDeletePoolRanking;
    toJSON(message: MsgDeletePoolRanking): unknown;
    fromPartial(object: DeepPartial<MsgDeletePoolRanking>): MsgDeletePoolRanking;
};
export declare const MsgDeletePoolRankingResponse: {
    encode(_: MsgDeletePoolRankingResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolRankingResponse;
    fromJSON(_: any): MsgDeletePoolRankingResponse;
    toJSON(_: MsgDeletePoolRankingResponse): unknown;
    fromPartial(_: DeepPartial<MsgDeletePoolRankingResponse>): MsgDeletePoolRankingResponse;
};
export declare const MsgCreatePoolSpotPrice: {
    encode(message: MsgCreatePoolSpotPrice, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolSpotPrice;
    fromJSON(object: any): MsgCreatePoolSpotPrice;
    toJSON(message: MsgCreatePoolSpotPrice): unknown;
    fromPartial(object: DeepPartial<MsgCreatePoolSpotPrice>): MsgCreatePoolSpotPrice;
};
export declare const MsgCreatePoolSpotPriceResponse: {
    encode(_: MsgCreatePoolSpotPriceResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolSpotPriceResponse;
    fromJSON(_: any): MsgCreatePoolSpotPriceResponse;
    toJSON(_: MsgCreatePoolSpotPriceResponse): unknown;
    fromPartial(_: DeepPartial<MsgCreatePoolSpotPriceResponse>): MsgCreatePoolSpotPriceResponse;
};
export declare const MsgUpdatePoolSpotPrice: {
    encode(message: MsgUpdatePoolSpotPrice, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolSpotPrice;
    fromJSON(object: any): MsgUpdatePoolSpotPrice;
    toJSON(message: MsgUpdatePoolSpotPrice): unknown;
    fromPartial(object: DeepPartial<MsgUpdatePoolSpotPrice>): MsgUpdatePoolSpotPrice;
};
export declare const MsgUpdatePoolSpotPriceResponse: {
    encode(_: MsgUpdatePoolSpotPriceResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolSpotPriceResponse;
    fromJSON(_: any): MsgUpdatePoolSpotPriceResponse;
    toJSON(_: MsgUpdatePoolSpotPriceResponse): unknown;
    fromPartial(_: DeepPartial<MsgUpdatePoolSpotPriceResponse>): MsgUpdatePoolSpotPriceResponse;
};
export declare const MsgDeletePoolSpotPrice: {
    encode(message: MsgDeletePoolSpotPrice, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolSpotPrice;
    fromJSON(object: any): MsgDeletePoolSpotPrice;
    toJSON(message: MsgDeletePoolSpotPrice): unknown;
    fromPartial(object: DeepPartial<MsgDeletePoolSpotPrice>): MsgDeletePoolSpotPrice;
};
export declare const MsgDeletePoolSpotPriceResponse: {
    encode(_: MsgDeletePoolSpotPriceResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolSpotPriceResponse;
    fromJSON(_: any): MsgDeletePoolSpotPriceResponse;
    toJSON(_: MsgDeletePoolSpotPriceResponse): unknown;
    fromPartial(_: DeepPartial<MsgDeletePoolSpotPriceResponse>): MsgDeletePoolSpotPriceResponse;
};
export declare const MsgCreatePoolInfo: {
    encode(message: MsgCreatePoolInfo, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolInfo;
    fromJSON(object: any): MsgCreatePoolInfo;
    toJSON(message: MsgCreatePoolInfo): unknown;
    fromPartial(object: DeepPartial<MsgCreatePoolInfo>): MsgCreatePoolInfo;
};
export declare const MsgCreatePoolInfoResponse: {
    encode(_: MsgCreatePoolInfoResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolInfoResponse;
    fromJSON(_: any): MsgCreatePoolInfoResponse;
    toJSON(_: MsgCreatePoolInfoResponse): unknown;
    fromPartial(_: DeepPartial<MsgCreatePoolInfoResponse>): MsgCreatePoolInfoResponse;
};
export declare const MsgUpdatePoolInfo: {
    encode(message: MsgUpdatePoolInfo, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolInfo;
    fromJSON(object: any): MsgUpdatePoolInfo;
    toJSON(message: MsgUpdatePoolInfo): unknown;
    fromPartial(object: DeepPartial<MsgUpdatePoolInfo>): MsgUpdatePoolInfo;
};
export declare const MsgUpdatePoolInfoResponse: {
    encode(_: MsgUpdatePoolInfoResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolInfoResponse;
    fromJSON(_: any): MsgUpdatePoolInfoResponse;
    toJSON(_: MsgUpdatePoolInfoResponse): unknown;
    fromPartial(_: DeepPartial<MsgUpdatePoolInfoResponse>): MsgUpdatePoolInfoResponse;
};
export declare const MsgDeletePoolInfo: {
    encode(message: MsgDeletePoolInfo, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolInfo;
    fromJSON(object: any): MsgDeletePoolInfo;
    toJSON(message: MsgDeletePoolInfo): unknown;
    fromPartial(object: DeepPartial<MsgDeletePoolInfo>): MsgDeletePoolInfo;
};
export declare const MsgDeletePoolInfoResponse: {
    encode(_: MsgDeletePoolInfoResponse, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolInfoResponse;
    fromJSON(_: any): MsgDeletePoolInfoResponse;
    toJSON(_: MsgDeletePoolInfoResponse): unknown;
    fromPartial(_: DeepPartial<MsgDeletePoolInfoResponse>): MsgDeletePoolInfoResponse;
};
/** Msg defines the Msg service. */
export interface Msg {
    CreatePoolPosition(request: MsgCreatePoolPosition): Promise<MsgCreatePoolPositionResponse>;
    UpdatePoolPosition(request: MsgUpdatePoolPosition): Promise<MsgUpdatePoolPositionResponse>;
    DeletePoolPosition(request: MsgDeletePoolPosition): Promise<MsgDeletePoolPositionResponse>;
    CreatePoolRanking(request: MsgCreatePoolRanking): Promise<MsgCreatePoolRankingResponse>;
    UpdatePoolRanking(request: MsgUpdatePoolRanking): Promise<MsgUpdatePoolRankingResponse>;
    DeletePoolRanking(request: MsgDeletePoolRanking): Promise<MsgDeletePoolRankingResponse>;
    CreatePoolSpotPrice(request: MsgCreatePoolSpotPrice): Promise<MsgCreatePoolSpotPriceResponse>;
    UpdatePoolSpotPrice(request: MsgUpdatePoolSpotPrice): Promise<MsgUpdatePoolSpotPriceResponse>;
    DeletePoolSpotPrice(request: MsgDeletePoolSpotPrice): Promise<MsgDeletePoolSpotPriceResponse>;
    CreatePoolInfo(request: MsgCreatePoolInfo): Promise<MsgCreatePoolInfoResponse>;
    UpdatePoolInfo(request: MsgUpdatePoolInfo): Promise<MsgUpdatePoolInfoResponse>;
    /** this line is used by starport scaffolding # proto/tx/rpc */
    DeletePoolInfo(request: MsgDeletePoolInfo): Promise<MsgDeletePoolInfoResponse>;
}
export declare class MsgClientImpl implements Msg {
    private readonly rpc;
    constructor(rpc: Rpc);
    CreatePoolPosition(request: MsgCreatePoolPosition): Promise<MsgCreatePoolPositionResponse>;
    UpdatePoolPosition(request: MsgUpdatePoolPosition): Promise<MsgUpdatePoolPositionResponse>;
    DeletePoolPosition(request: MsgDeletePoolPosition): Promise<MsgDeletePoolPositionResponse>;
    CreatePoolRanking(request: MsgCreatePoolRanking): Promise<MsgCreatePoolRankingResponse>;
    UpdatePoolRanking(request: MsgUpdatePoolRanking): Promise<MsgUpdatePoolRankingResponse>;
    DeletePoolRanking(request: MsgDeletePoolRanking): Promise<MsgDeletePoolRankingResponse>;
    CreatePoolSpotPrice(request: MsgCreatePoolSpotPrice): Promise<MsgCreatePoolSpotPriceResponse>;
    UpdatePoolSpotPrice(request: MsgUpdatePoolSpotPrice): Promise<MsgUpdatePoolSpotPriceResponse>;
    DeletePoolSpotPrice(request: MsgDeletePoolSpotPrice): Promise<MsgDeletePoolSpotPriceResponse>;
    CreatePoolInfo(request: MsgCreatePoolInfo): Promise<MsgCreatePoolInfoResponse>;
    UpdatePoolInfo(request: MsgUpdatePoolInfo): Promise<MsgUpdatePoolInfoResponse>;
    DeletePoolInfo(request: MsgDeletePoolInfo): Promise<MsgDeletePoolInfoResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
