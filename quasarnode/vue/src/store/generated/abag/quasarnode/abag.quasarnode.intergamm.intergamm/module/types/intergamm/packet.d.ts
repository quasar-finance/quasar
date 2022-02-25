import { Writer, Reader } from "protobufjs/minimal";
import { BalancerPoolParams } from "../osmosis/gamm/pool-models/balancer/balancerPool";
import { PoolAsset } from "../osmosis/gamm/v1beta1/pool";
import { Coin } from "../cosmos/base/v1beta1/coin";
export declare const protobufPackage = "intergamm";
export interface IntergammPacketData {
    noData: NoData | undefined;
    ibcCreatePoolPacket: IbcCreatePoolPacketData | undefined;
    ibcJoinPoolPacket: IbcJoinPoolPacketData | undefined;
    ibcExitPoolPacket: IbcExitPoolPacketData | undefined;
    /** this line is used by starport scaffolding # ibc/packet/proto/field */
    ibcWithdrawPacket: IbcWithdrawPacketData | undefined;
}
export interface NoData {
}
/** IbcCreatePoolPacketData defines a struct for the packet payload */
export interface IbcCreatePoolPacketData {
    params: BalancerPoolParams | undefined;
    /**
     * repeated abag.quasarnode.osmosis.gamm.v1beta1.PoolAsset assets = 2
     *  [ (gogoproto.nullable) = false ];
     */
    assets: PoolAsset[];
    futurePoolGovernor: string;
}
/** IbcCreatePoolPacketAck defines a struct for the packet acknowledgment */
export interface IbcCreatePoolPacketAck {
    poolId: number;
}
/** IbcJoinPoolPacketData defines a struct for the packet payload */
export interface IbcJoinPoolPacketData {
    poolId: number;
    shareOutAmount: string;
    tokenInMaxs: Coin[];
}
export interface IbcJoinPoolPacketAck {
}
export interface IbcExitPoolPacketData {
    poolId: number;
    shareInAmount: string;
    tokenOutMins: Coin[];
}
export interface IbcExitPoolPacketAck {
}
/** IbcWithdrawPacketData defines a struct for the packet payload */
export interface IbcWithdrawPacketData {
    transferPort: string;
    transferChannel: string;
    receiver: string;
    assets: Coin[];
}
/** IbcWithdrawPacketAck defines a struct for the packet acknowledgment */
export interface IbcWithdrawPacketAck {
}
export declare const IntergammPacketData: {
    encode(message: IntergammPacketData, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IntergammPacketData;
    fromJSON(object: any): IntergammPacketData;
    toJSON(message: IntergammPacketData): unknown;
    fromPartial(object: DeepPartial<IntergammPacketData>): IntergammPacketData;
};
export declare const NoData: {
    encode(_: NoData, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): NoData;
    fromJSON(_: any): NoData;
    toJSON(_: NoData): unknown;
    fromPartial(_: DeepPartial<NoData>): NoData;
};
export declare const IbcCreatePoolPacketData: {
    encode(message: IbcCreatePoolPacketData, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcCreatePoolPacketData;
    fromJSON(object: any): IbcCreatePoolPacketData;
    toJSON(message: IbcCreatePoolPacketData): unknown;
    fromPartial(object: DeepPartial<IbcCreatePoolPacketData>): IbcCreatePoolPacketData;
};
export declare const IbcCreatePoolPacketAck: {
    encode(message: IbcCreatePoolPacketAck, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcCreatePoolPacketAck;
    fromJSON(object: any): IbcCreatePoolPacketAck;
    toJSON(message: IbcCreatePoolPacketAck): unknown;
    fromPartial(object: DeepPartial<IbcCreatePoolPacketAck>): IbcCreatePoolPacketAck;
};
export declare const IbcJoinPoolPacketData: {
    encode(message: IbcJoinPoolPacketData, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcJoinPoolPacketData;
    fromJSON(object: any): IbcJoinPoolPacketData;
    toJSON(message: IbcJoinPoolPacketData): unknown;
    fromPartial(object: DeepPartial<IbcJoinPoolPacketData>): IbcJoinPoolPacketData;
};
export declare const IbcJoinPoolPacketAck: {
    encode(_: IbcJoinPoolPacketAck, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcJoinPoolPacketAck;
    fromJSON(_: any): IbcJoinPoolPacketAck;
    toJSON(_: IbcJoinPoolPacketAck): unknown;
    fromPartial(_: DeepPartial<IbcJoinPoolPacketAck>): IbcJoinPoolPacketAck;
};
export declare const IbcExitPoolPacketData: {
    encode(message: IbcExitPoolPacketData, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcExitPoolPacketData;
    fromJSON(object: any): IbcExitPoolPacketData;
    toJSON(message: IbcExitPoolPacketData): unknown;
    fromPartial(object: DeepPartial<IbcExitPoolPacketData>): IbcExitPoolPacketData;
};
export declare const IbcExitPoolPacketAck: {
    encode(_: IbcExitPoolPacketAck, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcExitPoolPacketAck;
    fromJSON(_: any): IbcExitPoolPacketAck;
    toJSON(_: IbcExitPoolPacketAck): unknown;
    fromPartial(_: DeepPartial<IbcExitPoolPacketAck>): IbcExitPoolPacketAck;
};
export declare const IbcWithdrawPacketData: {
    encode(message: IbcWithdrawPacketData, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcWithdrawPacketData;
    fromJSON(object: any): IbcWithdrawPacketData;
    toJSON(message: IbcWithdrawPacketData): unknown;
    fromPartial(object: DeepPartial<IbcWithdrawPacketData>): IbcWithdrawPacketData;
};
export declare const IbcWithdrawPacketAck: {
    encode(_: IbcWithdrawPacketAck, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): IbcWithdrawPacketAck;
    fromJSON(_: any): IbcWithdrawPacketAck;
    toJSON(_: IbcWithdrawPacketAck): unknown;
    fromPartial(_: DeepPartial<IbcWithdrawPacketAck>): IbcWithdrawPacketAck;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
