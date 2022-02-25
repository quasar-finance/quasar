/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { BalancerPoolParams } from "../osmosis/gamm/pool-models/balancer/balancerPool";
import { PoolAsset } from "../osmosis/gamm/v1beta1/pool";
import { Coin } from "../cosmos/base/v1beta1/coin";

export const protobufPackage = "intergamm";

export interface IntergammPacketData {
  noData: NoData | undefined;
  ibcCreatePoolPacket: IbcCreatePoolPacketData | undefined;
  ibcJoinPoolPacket: IbcJoinPoolPacketData | undefined;
  ibcExitPoolPacket: IbcExitPoolPacketData | undefined;
  /** this line is used by starport scaffolding # ibc/packet/proto/field */
  ibcWithdrawPacket: IbcWithdrawPacketData | undefined;
}

export interface NoData {}

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

export interface IbcJoinPoolPacketAck {}

export interface IbcExitPoolPacketData {
  poolId: number;
  shareInAmount: string;
  tokenOutMins: Coin[];
}

export interface IbcExitPoolPacketAck {}

/** IbcWithdrawPacketData defines a struct for the packet payload */
export interface IbcWithdrawPacketData {
  transferPort: string;
  transferChannel: string;
  receiver: string;
  assets: Coin[];
}

/** IbcWithdrawPacketAck defines a struct for the packet acknowledgment */
export interface IbcWithdrawPacketAck {}

const baseIntergammPacketData: object = {};

export const IntergammPacketData = {
  encode(
    message: IntergammPacketData,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.noData !== undefined) {
      NoData.encode(message.noData, writer.uint32(10).fork()).ldelim();
    }
    if (message.ibcCreatePoolPacket !== undefined) {
      IbcCreatePoolPacketData.encode(
        message.ibcCreatePoolPacket,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.ibcJoinPoolPacket !== undefined) {
      IbcJoinPoolPacketData.encode(
        message.ibcJoinPoolPacket,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.ibcExitPoolPacket !== undefined) {
      IbcExitPoolPacketData.encode(
        message.ibcExitPoolPacket,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.ibcWithdrawPacket !== undefined) {
      IbcWithdrawPacketData.encode(
        message.ibcWithdrawPacket,
        writer.uint32(42).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IntergammPacketData {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIntergammPacketData } as IntergammPacketData;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.noData = NoData.decode(reader, reader.uint32());
          break;
        case 2:
          message.ibcCreatePoolPacket = IbcCreatePoolPacketData.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.ibcJoinPoolPacket = IbcJoinPoolPacketData.decode(
            reader,
            reader.uint32()
          );
          break;
        case 4:
          message.ibcExitPoolPacket = IbcExitPoolPacketData.decode(
            reader,
            reader.uint32()
          );
          break;
        case 5:
          message.ibcWithdrawPacket = IbcWithdrawPacketData.decode(
            reader,
            reader.uint32()
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): IntergammPacketData {
    const message = { ...baseIntergammPacketData } as IntergammPacketData;
    if (object.noData !== undefined && object.noData !== null) {
      message.noData = NoData.fromJSON(object.noData);
    } else {
      message.noData = undefined;
    }
    if (
      object.ibcCreatePoolPacket !== undefined &&
      object.ibcCreatePoolPacket !== null
    ) {
      message.ibcCreatePoolPacket = IbcCreatePoolPacketData.fromJSON(
        object.ibcCreatePoolPacket
      );
    } else {
      message.ibcCreatePoolPacket = undefined;
    }
    if (
      object.ibcJoinPoolPacket !== undefined &&
      object.ibcJoinPoolPacket !== null
    ) {
      message.ibcJoinPoolPacket = IbcJoinPoolPacketData.fromJSON(
        object.ibcJoinPoolPacket
      );
    } else {
      message.ibcJoinPoolPacket = undefined;
    }
    if (
      object.ibcExitPoolPacket !== undefined &&
      object.ibcExitPoolPacket !== null
    ) {
      message.ibcExitPoolPacket = IbcExitPoolPacketData.fromJSON(
        object.ibcExitPoolPacket
      );
    } else {
      message.ibcExitPoolPacket = undefined;
    }
    if (
      object.ibcWithdrawPacket !== undefined &&
      object.ibcWithdrawPacket !== null
    ) {
      message.ibcWithdrawPacket = IbcWithdrawPacketData.fromJSON(
        object.ibcWithdrawPacket
      );
    } else {
      message.ibcWithdrawPacket = undefined;
    }
    return message;
  },

  toJSON(message: IntergammPacketData): unknown {
    const obj: any = {};
    message.noData !== undefined &&
      (obj.noData = message.noData ? NoData.toJSON(message.noData) : undefined);
    message.ibcCreatePoolPacket !== undefined &&
      (obj.ibcCreatePoolPacket = message.ibcCreatePoolPacket
        ? IbcCreatePoolPacketData.toJSON(message.ibcCreatePoolPacket)
        : undefined);
    message.ibcJoinPoolPacket !== undefined &&
      (obj.ibcJoinPoolPacket = message.ibcJoinPoolPacket
        ? IbcJoinPoolPacketData.toJSON(message.ibcJoinPoolPacket)
        : undefined);
    message.ibcExitPoolPacket !== undefined &&
      (obj.ibcExitPoolPacket = message.ibcExitPoolPacket
        ? IbcExitPoolPacketData.toJSON(message.ibcExitPoolPacket)
        : undefined);
    message.ibcWithdrawPacket !== undefined &&
      (obj.ibcWithdrawPacket = message.ibcWithdrawPacket
        ? IbcWithdrawPacketData.toJSON(message.ibcWithdrawPacket)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<IntergammPacketData>): IntergammPacketData {
    const message = { ...baseIntergammPacketData } as IntergammPacketData;
    if (object.noData !== undefined && object.noData !== null) {
      message.noData = NoData.fromPartial(object.noData);
    } else {
      message.noData = undefined;
    }
    if (
      object.ibcCreatePoolPacket !== undefined &&
      object.ibcCreatePoolPacket !== null
    ) {
      message.ibcCreatePoolPacket = IbcCreatePoolPacketData.fromPartial(
        object.ibcCreatePoolPacket
      );
    } else {
      message.ibcCreatePoolPacket = undefined;
    }
    if (
      object.ibcJoinPoolPacket !== undefined &&
      object.ibcJoinPoolPacket !== null
    ) {
      message.ibcJoinPoolPacket = IbcJoinPoolPacketData.fromPartial(
        object.ibcJoinPoolPacket
      );
    } else {
      message.ibcJoinPoolPacket = undefined;
    }
    if (
      object.ibcExitPoolPacket !== undefined &&
      object.ibcExitPoolPacket !== null
    ) {
      message.ibcExitPoolPacket = IbcExitPoolPacketData.fromPartial(
        object.ibcExitPoolPacket
      );
    } else {
      message.ibcExitPoolPacket = undefined;
    }
    if (
      object.ibcWithdrawPacket !== undefined &&
      object.ibcWithdrawPacket !== null
    ) {
      message.ibcWithdrawPacket = IbcWithdrawPacketData.fromPartial(
        object.ibcWithdrawPacket
      );
    } else {
      message.ibcWithdrawPacket = undefined;
    }
    return message;
  },
};

const baseNoData: object = {};

export const NoData = {
  encode(_: NoData, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): NoData {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseNoData } as NoData;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): NoData {
    const message = { ...baseNoData } as NoData;
    return message;
  },

  toJSON(_: NoData): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<NoData>): NoData {
    const message = { ...baseNoData } as NoData;
    return message;
  },
};

const baseIbcCreatePoolPacketData: object = { futurePoolGovernor: "" };

export const IbcCreatePoolPacketData = {
  encode(
    message: IbcCreatePoolPacketData,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.params !== undefined) {
      BalancerPoolParams.encode(
        message.params,
        writer.uint32(10).fork()
      ).ldelim();
    }
    for (const v of message.assets) {
      PoolAsset.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.futurePoolGovernor !== "") {
      writer.uint32(26).string(message.futurePoolGovernor);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcCreatePoolPacketData {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseIbcCreatePoolPacketData,
    } as IbcCreatePoolPacketData;
    message.assets = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.params = BalancerPoolParams.decode(reader, reader.uint32());
          break;
        case 2:
          message.assets.push(PoolAsset.decode(reader, reader.uint32()));
          break;
        case 3:
          message.futurePoolGovernor = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): IbcCreatePoolPacketData {
    const message = {
      ...baseIbcCreatePoolPacketData,
    } as IbcCreatePoolPacketData;
    message.assets = [];
    if (object.params !== undefined && object.params !== null) {
      message.params = BalancerPoolParams.fromJSON(object.params);
    } else {
      message.params = undefined;
    }
    if (object.assets !== undefined && object.assets !== null) {
      for (const e of object.assets) {
        message.assets.push(PoolAsset.fromJSON(e));
      }
    }
    if (
      object.futurePoolGovernor !== undefined &&
      object.futurePoolGovernor !== null
    ) {
      message.futurePoolGovernor = String(object.futurePoolGovernor);
    } else {
      message.futurePoolGovernor = "";
    }
    return message;
  },

  toJSON(message: IbcCreatePoolPacketData): unknown {
    const obj: any = {};
    message.params !== undefined &&
      (obj.params = message.params
        ? BalancerPoolParams.toJSON(message.params)
        : undefined);
    if (message.assets) {
      obj.assets = message.assets.map((e) =>
        e ? PoolAsset.toJSON(e) : undefined
      );
    } else {
      obj.assets = [];
    }
    message.futurePoolGovernor !== undefined &&
      (obj.futurePoolGovernor = message.futurePoolGovernor);
    return obj;
  },

  fromPartial(
    object: DeepPartial<IbcCreatePoolPacketData>
  ): IbcCreatePoolPacketData {
    const message = {
      ...baseIbcCreatePoolPacketData,
    } as IbcCreatePoolPacketData;
    message.assets = [];
    if (object.params !== undefined && object.params !== null) {
      message.params = BalancerPoolParams.fromPartial(object.params);
    } else {
      message.params = undefined;
    }
    if (object.assets !== undefined && object.assets !== null) {
      for (const e of object.assets) {
        message.assets.push(PoolAsset.fromPartial(e));
      }
    }
    if (
      object.futurePoolGovernor !== undefined &&
      object.futurePoolGovernor !== null
    ) {
      message.futurePoolGovernor = object.futurePoolGovernor;
    } else {
      message.futurePoolGovernor = "";
    }
    return message;
  },
};

const baseIbcCreatePoolPacketAck: object = { poolId: 0 };

export const IbcCreatePoolPacketAck = {
  encode(
    message: IbcCreatePoolPacketAck,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolId !== 0) {
      writer.uint32(8).uint64(message.poolId);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcCreatePoolPacketAck {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIbcCreatePoolPacketAck } as IbcCreatePoolPacketAck;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolId = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): IbcCreatePoolPacketAck {
    const message = { ...baseIbcCreatePoolPacketAck } as IbcCreatePoolPacketAck;
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = Number(object.poolId);
    } else {
      message.poolId = 0;
    }
    return message;
  },

  toJSON(message: IbcCreatePoolPacketAck): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    return obj;
  },

  fromPartial(
    object: DeepPartial<IbcCreatePoolPacketAck>
  ): IbcCreatePoolPacketAck {
    const message = { ...baseIbcCreatePoolPacketAck } as IbcCreatePoolPacketAck;
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = 0;
    }
    return message;
  },
};

const baseIbcJoinPoolPacketData: object = { poolId: 0, shareOutAmount: "" };

export const IbcJoinPoolPacketData = {
  encode(
    message: IbcJoinPoolPacketData,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolId !== 0) {
      writer.uint32(16).uint64(message.poolId);
    }
    if (message.shareOutAmount !== "") {
      writer.uint32(26).string(message.shareOutAmount);
    }
    for (const v of message.tokenInMaxs) {
      Coin.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcJoinPoolPacketData {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIbcJoinPoolPacketData } as IbcJoinPoolPacketData;
    message.tokenInMaxs = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.poolId = longToNumber(reader.uint64() as Long);
          break;
        case 3:
          message.shareOutAmount = reader.string();
          break;
        case 4:
          message.tokenInMaxs.push(Coin.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): IbcJoinPoolPacketData {
    const message = { ...baseIbcJoinPoolPacketData } as IbcJoinPoolPacketData;
    message.tokenInMaxs = [];
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = Number(object.poolId);
    } else {
      message.poolId = 0;
    }
    if (object.shareOutAmount !== undefined && object.shareOutAmount !== null) {
      message.shareOutAmount = String(object.shareOutAmount);
    } else {
      message.shareOutAmount = "";
    }
    if (object.tokenInMaxs !== undefined && object.tokenInMaxs !== null) {
      for (const e of object.tokenInMaxs) {
        message.tokenInMaxs.push(Coin.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: IbcJoinPoolPacketData): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.shareOutAmount !== undefined &&
      (obj.shareOutAmount = message.shareOutAmount);
    if (message.tokenInMaxs) {
      obj.tokenInMaxs = message.tokenInMaxs.map((e) =>
        e ? Coin.toJSON(e) : undefined
      );
    } else {
      obj.tokenInMaxs = [];
    }
    return obj;
  },

  fromPartial(
    object: DeepPartial<IbcJoinPoolPacketData>
  ): IbcJoinPoolPacketData {
    const message = { ...baseIbcJoinPoolPacketData } as IbcJoinPoolPacketData;
    message.tokenInMaxs = [];
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = 0;
    }
    if (object.shareOutAmount !== undefined && object.shareOutAmount !== null) {
      message.shareOutAmount = object.shareOutAmount;
    } else {
      message.shareOutAmount = "";
    }
    if (object.tokenInMaxs !== undefined && object.tokenInMaxs !== null) {
      for (const e of object.tokenInMaxs) {
        message.tokenInMaxs.push(Coin.fromPartial(e));
      }
    }
    return message;
  },
};

const baseIbcJoinPoolPacketAck: object = {};

export const IbcJoinPoolPacketAck = {
  encode(_: IbcJoinPoolPacketAck, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcJoinPoolPacketAck {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIbcJoinPoolPacketAck } as IbcJoinPoolPacketAck;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): IbcJoinPoolPacketAck {
    const message = { ...baseIbcJoinPoolPacketAck } as IbcJoinPoolPacketAck;
    return message;
  },

  toJSON(_: IbcJoinPoolPacketAck): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<IbcJoinPoolPacketAck>): IbcJoinPoolPacketAck {
    const message = { ...baseIbcJoinPoolPacketAck } as IbcJoinPoolPacketAck;
    return message;
  },
};

const baseIbcExitPoolPacketData: object = { poolId: 0, shareInAmount: "" };

export const IbcExitPoolPacketData = {
  encode(
    message: IbcExitPoolPacketData,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolId !== 0) {
      writer.uint32(16).uint64(message.poolId);
    }
    if (message.shareInAmount !== "") {
      writer.uint32(26).string(message.shareInAmount);
    }
    for (const v of message.tokenOutMins) {
      Coin.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcExitPoolPacketData {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIbcExitPoolPacketData } as IbcExitPoolPacketData;
    message.tokenOutMins = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.poolId = longToNumber(reader.uint64() as Long);
          break;
        case 3:
          message.shareInAmount = reader.string();
          break;
        case 4:
          message.tokenOutMins.push(Coin.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): IbcExitPoolPacketData {
    const message = { ...baseIbcExitPoolPacketData } as IbcExitPoolPacketData;
    message.tokenOutMins = [];
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = Number(object.poolId);
    } else {
      message.poolId = 0;
    }
    if (object.shareInAmount !== undefined && object.shareInAmount !== null) {
      message.shareInAmount = String(object.shareInAmount);
    } else {
      message.shareInAmount = "";
    }
    if (object.tokenOutMins !== undefined && object.tokenOutMins !== null) {
      for (const e of object.tokenOutMins) {
        message.tokenOutMins.push(Coin.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: IbcExitPoolPacketData): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.shareInAmount !== undefined &&
      (obj.shareInAmount = message.shareInAmount);
    if (message.tokenOutMins) {
      obj.tokenOutMins = message.tokenOutMins.map((e) =>
        e ? Coin.toJSON(e) : undefined
      );
    } else {
      obj.tokenOutMins = [];
    }
    return obj;
  },

  fromPartial(
    object: DeepPartial<IbcExitPoolPacketData>
  ): IbcExitPoolPacketData {
    const message = { ...baseIbcExitPoolPacketData } as IbcExitPoolPacketData;
    message.tokenOutMins = [];
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = 0;
    }
    if (object.shareInAmount !== undefined && object.shareInAmount !== null) {
      message.shareInAmount = object.shareInAmount;
    } else {
      message.shareInAmount = "";
    }
    if (object.tokenOutMins !== undefined && object.tokenOutMins !== null) {
      for (const e of object.tokenOutMins) {
        message.tokenOutMins.push(Coin.fromPartial(e));
      }
    }
    return message;
  },
};

const baseIbcExitPoolPacketAck: object = {};

export const IbcExitPoolPacketAck = {
  encode(_: IbcExitPoolPacketAck, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcExitPoolPacketAck {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIbcExitPoolPacketAck } as IbcExitPoolPacketAck;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): IbcExitPoolPacketAck {
    const message = { ...baseIbcExitPoolPacketAck } as IbcExitPoolPacketAck;
    return message;
  },

  toJSON(_: IbcExitPoolPacketAck): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<IbcExitPoolPacketAck>): IbcExitPoolPacketAck {
    const message = { ...baseIbcExitPoolPacketAck } as IbcExitPoolPacketAck;
    return message;
  },
};

const baseIbcWithdrawPacketData: object = {
  transferPort: "",
  transferChannel: "",
  receiver: "",
};

export const IbcWithdrawPacketData = {
  encode(
    message: IbcWithdrawPacketData,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.transferPort !== "") {
      writer.uint32(10).string(message.transferPort);
    }
    if (message.transferChannel !== "") {
      writer.uint32(18).string(message.transferChannel);
    }
    if (message.receiver !== "") {
      writer.uint32(26).string(message.receiver);
    }
    for (const v of message.assets) {
      Coin.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcWithdrawPacketData {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIbcWithdrawPacketData } as IbcWithdrawPacketData;
    message.assets = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.transferPort = reader.string();
          break;
        case 2:
          message.transferChannel = reader.string();
          break;
        case 3:
          message.receiver = reader.string();
          break;
        case 4:
          message.assets.push(Coin.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): IbcWithdrawPacketData {
    const message = { ...baseIbcWithdrawPacketData } as IbcWithdrawPacketData;
    message.assets = [];
    if (object.transferPort !== undefined && object.transferPort !== null) {
      message.transferPort = String(object.transferPort);
    } else {
      message.transferPort = "";
    }
    if (
      object.transferChannel !== undefined &&
      object.transferChannel !== null
    ) {
      message.transferChannel = String(object.transferChannel);
    } else {
      message.transferChannel = "";
    }
    if (object.receiver !== undefined && object.receiver !== null) {
      message.receiver = String(object.receiver);
    } else {
      message.receiver = "";
    }
    if (object.assets !== undefined && object.assets !== null) {
      for (const e of object.assets) {
        message.assets.push(Coin.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: IbcWithdrawPacketData): unknown {
    const obj: any = {};
    message.transferPort !== undefined &&
      (obj.transferPort = message.transferPort);
    message.transferChannel !== undefined &&
      (obj.transferChannel = message.transferChannel);
    message.receiver !== undefined && (obj.receiver = message.receiver);
    if (message.assets) {
      obj.assets = message.assets.map((e) => (e ? Coin.toJSON(e) : undefined));
    } else {
      obj.assets = [];
    }
    return obj;
  },

  fromPartial(
    object: DeepPartial<IbcWithdrawPacketData>
  ): IbcWithdrawPacketData {
    const message = { ...baseIbcWithdrawPacketData } as IbcWithdrawPacketData;
    message.assets = [];
    if (object.transferPort !== undefined && object.transferPort !== null) {
      message.transferPort = object.transferPort;
    } else {
      message.transferPort = "";
    }
    if (
      object.transferChannel !== undefined &&
      object.transferChannel !== null
    ) {
      message.transferChannel = object.transferChannel;
    } else {
      message.transferChannel = "";
    }
    if (object.receiver !== undefined && object.receiver !== null) {
      message.receiver = object.receiver;
    } else {
      message.receiver = "";
    }
    if (object.assets !== undefined && object.assets !== null) {
      for (const e of object.assets) {
        message.assets.push(Coin.fromPartial(e));
      }
    }
    return message;
  },
};

const baseIbcWithdrawPacketAck: object = {};

export const IbcWithdrawPacketAck = {
  encode(_: IbcWithdrawPacketAck, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): IbcWithdrawPacketAck {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseIbcWithdrawPacketAck } as IbcWithdrawPacketAck;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): IbcWithdrawPacketAck {
    const message = { ...baseIbcWithdrawPacketAck } as IbcWithdrawPacketAck;
    return message;
  },

  toJSON(_: IbcWithdrawPacketAck): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<IbcWithdrawPacketAck>): IbcWithdrawPacketAck {
    const message = { ...baseIbcWithdrawPacketAck } as IbcWithdrawPacketAck;
    return message;
  },
};

declare var self: any | undefined;
declare var window: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") return globalThis;
  if (typeof self !== "undefined") return self;
  if (typeof window !== "undefined") return window;
  if (typeof global !== "undefined") return global;
  throw "Unable to locate global object";
})();

type Builtin = Date | Function | Uint8Array | string | number | undefined;
export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (util.Long !== Long) {
  util.Long = Long as any;
  configure();
}
