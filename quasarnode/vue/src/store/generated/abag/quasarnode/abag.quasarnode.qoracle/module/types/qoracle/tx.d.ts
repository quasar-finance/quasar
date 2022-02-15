import { Reader, Writer } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface MsgCreatePoolPosition {
    creator: string;
    poolID: number;
    aPY: number;
    tVL: number;
    lastUpdatedTime: number;
}
export interface MsgCreatePoolPositionResponse {
}
export interface MsgUpdatePoolPosition {
    creator: string;
    poolID: number;
    aPY: number;
    tVL: number;
    lastUpdatedTime: number;
}
export interface MsgUpdatePoolPositionResponse {
}
export interface MsgDeletePoolPosition {
    creator: string;
    poolID: number;
}
export interface MsgDeletePoolPositionResponse {
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
/** Msg defines the Msg service. */
export interface Msg {
    CreatePoolPosition(request: MsgCreatePoolPosition): Promise<MsgCreatePoolPositionResponse>;
    UpdatePoolPosition(request: MsgUpdatePoolPosition): Promise<MsgUpdatePoolPositionResponse>;
    /** this line is used by starport scaffolding # proto/tx/rpc */
    DeletePoolPosition(request: MsgDeletePoolPosition): Promise<MsgDeletePoolPositionResponse>;
}
export declare class MsgClientImpl implements Msg {
    private readonly rpc;
    constructor(rpc: Rpc);
    CreatePoolPosition(request: MsgCreatePoolPosition): Promise<MsgCreatePoolPositionResponse>;
    UpdatePoolPosition(request: MsgUpdatePoolPosition): Promise<MsgUpdatePoolPositionResponse>;
    DeletePoolPosition(request: MsgDeletePoolPosition): Promise<MsgDeletePoolPositionResponse>;
}
interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
