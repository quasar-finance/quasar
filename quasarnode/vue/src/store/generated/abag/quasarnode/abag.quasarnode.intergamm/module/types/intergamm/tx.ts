/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { BalancerPoolParams } from "../osmosis/gamm/pool-models/balancer/balancerPool";
import { PoolAsset } from "../osmosis/gamm/v1beta1/pool";
import { Coin } from "../cosmos/base/v1beta1/coin";

export const protobufPackage = "abag.quasarnode.intergamm";

export interface MsgRegisterAccount {
  creator: string;
  connectionId: string;
}

export interface MsgRegisterAccountResponse {}

export interface MsgCreatePool {
  creator: string;
  connectionId: string;
  timeoutTimestamp: number;
  poolParams: BalancerPoolParams | undefined;
  poolAssets: PoolAsset[];
  futurePoolGovernor: string;
}

export interface MsgCreatePoolResponse {}

export interface MsgJoinPool {
  creator: string;
  connectionId: string;
  timeoutTimestamp: number;
  poolId: number;
  shareOutAmount: string;
  tokenInMaxs: Coin[];
}

export interface MsgJoinPoolResponse {}

export interface MsgExitPool {
  creator: string;
  connectionId: string;
  timeoutTimestamp: number;
  poolId: number;
  shareInAmount: string;
  tokenOutMins: Coin[];
}

export interface MsgExitPoolResponse {}

export interface MsgIbcTransfer {
  creator: string;
}

export interface MsgIbcTransferResponse {}

const baseMsgRegisterAccount: object = { creator: "", connectionId: "" };

export const MsgRegisterAccount = {
  encode(
    message: MsgRegisterAccount,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.connectionId !== "") {
      writer.uint32(18).string(message.connectionId);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgRegisterAccount {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgRegisterAccount } as MsgRegisterAccount;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.connectionId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgRegisterAccount {
    const message = { ...baseMsgRegisterAccount } as MsgRegisterAccount;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = String(object.connectionId);
    } else {
      message.connectionId = "";
    }
    return message;
  },

  toJSON(message: MsgRegisterAccount): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.connectionId !== undefined &&
      (obj.connectionId = message.connectionId);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgRegisterAccount>): MsgRegisterAccount {
    const message = { ...baseMsgRegisterAccount } as MsgRegisterAccount;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = object.connectionId;
    } else {
      message.connectionId = "";
    }
    return message;
  },
};

const baseMsgRegisterAccountResponse: object = {};

export const MsgRegisterAccountResponse = {
  encode(
    _: MsgRegisterAccountResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgRegisterAccountResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgRegisterAccountResponse,
    } as MsgRegisterAccountResponse;
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

  fromJSON(_: any): MsgRegisterAccountResponse {
    const message = {
      ...baseMsgRegisterAccountResponse,
    } as MsgRegisterAccountResponse;
    return message;
  },

  toJSON(_: MsgRegisterAccountResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgRegisterAccountResponse>
  ): MsgRegisterAccountResponse {
    const message = {
      ...baseMsgRegisterAccountResponse,
    } as MsgRegisterAccountResponse;
    return message;
  },
};

const baseMsgCreatePool: object = {
  creator: "",
  connectionId: "",
  timeoutTimestamp: 0,
  futurePoolGovernor: "",
};

export const MsgCreatePool = {
  encode(message: MsgCreatePool, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.connectionId !== "") {
      writer.uint32(18).string(message.connectionId);
    }
    if (message.timeoutTimestamp !== 0) {
      writer.uint32(24).uint64(message.timeoutTimestamp);
    }
    if (message.poolParams !== undefined) {
      BalancerPoolParams.encode(
        message.poolParams,
        writer.uint32(34).fork()
      ).ldelim();
    }
    for (const v of message.poolAssets) {
      PoolAsset.encode(v!, writer.uint32(42).fork()).ldelim();
    }
    if (message.futurePoolGovernor !== "") {
      writer.uint32(50).string(message.futurePoolGovernor);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgCreatePool {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgCreatePool } as MsgCreatePool;
    message.poolAssets = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.connectionId = reader.string();
          break;
        case 3:
          message.timeoutTimestamp = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.poolParams = BalancerPoolParams.decode(
            reader,
            reader.uint32()
          );
          break;
        case 5:
          message.poolAssets.push(PoolAsset.decode(reader, reader.uint32()));
          break;
        case 6:
          message.futurePoolGovernor = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgCreatePool {
    const message = { ...baseMsgCreatePool } as MsgCreatePool;
    message.poolAssets = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = String(object.connectionId);
    } else {
      message.connectionId = "";
    }
    if (
      object.timeoutTimestamp !== undefined &&
      object.timeoutTimestamp !== null
    ) {
      message.timeoutTimestamp = Number(object.timeoutTimestamp);
    } else {
      message.timeoutTimestamp = 0;
    }
    if (object.poolParams !== undefined && object.poolParams !== null) {
      message.poolParams = BalancerPoolParams.fromJSON(object.poolParams);
    } else {
      message.poolParams = undefined;
    }
    if (object.poolAssets !== undefined && object.poolAssets !== null) {
      for (const e of object.poolAssets) {
        message.poolAssets.push(PoolAsset.fromJSON(e));
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

  toJSON(message: MsgCreatePool): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.connectionId !== undefined &&
      (obj.connectionId = message.connectionId);
    message.timeoutTimestamp !== undefined &&
      (obj.timeoutTimestamp = message.timeoutTimestamp);
    message.poolParams !== undefined &&
      (obj.poolParams = message.poolParams
        ? BalancerPoolParams.toJSON(message.poolParams)
        : undefined);
    if (message.poolAssets) {
      obj.poolAssets = message.poolAssets.map((e) =>
        e ? PoolAsset.toJSON(e) : undefined
      );
    } else {
      obj.poolAssets = [];
    }
    message.futurePoolGovernor !== undefined &&
      (obj.futurePoolGovernor = message.futurePoolGovernor);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgCreatePool>): MsgCreatePool {
    const message = { ...baseMsgCreatePool } as MsgCreatePool;
    message.poolAssets = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = object.connectionId;
    } else {
      message.connectionId = "";
    }
    if (
      object.timeoutTimestamp !== undefined &&
      object.timeoutTimestamp !== null
    ) {
      message.timeoutTimestamp = object.timeoutTimestamp;
    } else {
      message.timeoutTimestamp = 0;
    }
    if (object.poolParams !== undefined && object.poolParams !== null) {
      message.poolParams = BalancerPoolParams.fromPartial(object.poolParams);
    } else {
      message.poolParams = undefined;
    }
    if (object.poolAssets !== undefined && object.poolAssets !== null) {
      for (const e of object.poolAssets) {
        message.poolAssets.push(PoolAsset.fromPartial(e));
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

const baseMsgCreatePoolResponse: object = {};

export const MsgCreatePoolResponse = {
  encode(_: MsgCreatePoolResponse, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgCreatePoolResponse } as MsgCreatePoolResponse;
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

  fromJSON(_: any): MsgCreatePoolResponse {
    const message = { ...baseMsgCreatePoolResponse } as MsgCreatePoolResponse;
    return message;
  },

  toJSON(_: MsgCreatePoolResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<MsgCreatePoolResponse>): MsgCreatePoolResponse {
    const message = { ...baseMsgCreatePoolResponse } as MsgCreatePoolResponse;
    return message;
  },
};

const baseMsgJoinPool: object = {
  creator: "",
  connectionId: "",
  timeoutTimestamp: 0,
  poolId: 0,
  shareOutAmount: "",
};

export const MsgJoinPool = {
  encode(message: MsgJoinPool, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.connectionId !== "") {
      writer.uint32(18).string(message.connectionId);
    }
    if (message.timeoutTimestamp !== 0) {
      writer.uint32(24).uint64(message.timeoutTimestamp);
    }
    if (message.poolId !== 0) {
      writer.uint32(32).uint64(message.poolId);
    }
    if (message.shareOutAmount !== "") {
      writer.uint32(42).string(message.shareOutAmount);
    }
    for (const v of message.tokenInMaxs) {
      Coin.encode(v!, writer.uint32(50).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgJoinPool {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgJoinPool } as MsgJoinPool;
    message.tokenInMaxs = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.connectionId = reader.string();
          break;
        case 3:
          message.timeoutTimestamp = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.poolId = longToNumber(reader.uint64() as Long);
          break;
        case 5:
          message.shareOutAmount = reader.string();
          break;
        case 6:
          message.tokenInMaxs.push(Coin.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgJoinPool {
    const message = { ...baseMsgJoinPool } as MsgJoinPool;
    message.tokenInMaxs = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = String(object.connectionId);
    } else {
      message.connectionId = "";
    }
    if (
      object.timeoutTimestamp !== undefined &&
      object.timeoutTimestamp !== null
    ) {
      message.timeoutTimestamp = Number(object.timeoutTimestamp);
    } else {
      message.timeoutTimestamp = 0;
    }
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

  toJSON(message: MsgJoinPool): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.connectionId !== undefined &&
      (obj.connectionId = message.connectionId);
    message.timeoutTimestamp !== undefined &&
      (obj.timeoutTimestamp = message.timeoutTimestamp);
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

  fromPartial(object: DeepPartial<MsgJoinPool>): MsgJoinPool {
    const message = { ...baseMsgJoinPool } as MsgJoinPool;
    message.tokenInMaxs = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = object.connectionId;
    } else {
      message.connectionId = "";
    }
    if (
      object.timeoutTimestamp !== undefined &&
      object.timeoutTimestamp !== null
    ) {
      message.timeoutTimestamp = object.timeoutTimestamp;
    } else {
      message.timeoutTimestamp = 0;
    }
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

const baseMsgJoinPoolResponse: object = {};

export const MsgJoinPoolResponse = {
  encode(_: MsgJoinPoolResponse, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgJoinPoolResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgJoinPoolResponse } as MsgJoinPoolResponse;
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

  fromJSON(_: any): MsgJoinPoolResponse {
    const message = { ...baseMsgJoinPoolResponse } as MsgJoinPoolResponse;
    return message;
  },

  toJSON(_: MsgJoinPoolResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<MsgJoinPoolResponse>): MsgJoinPoolResponse {
    const message = { ...baseMsgJoinPoolResponse } as MsgJoinPoolResponse;
    return message;
  },
};

const baseMsgExitPool: object = {
  creator: "",
  connectionId: "",
  timeoutTimestamp: 0,
  poolId: 0,
  shareInAmount: "",
};

export const MsgExitPool = {
  encode(message: MsgExitPool, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.connectionId !== "") {
      writer.uint32(18).string(message.connectionId);
    }
    if (message.timeoutTimestamp !== 0) {
      writer.uint32(24).uint64(message.timeoutTimestamp);
    }
    if (message.poolId !== 0) {
      writer.uint32(32).uint64(message.poolId);
    }
    if (message.shareInAmount !== "") {
      writer.uint32(42).string(message.shareInAmount);
    }
    for (const v of message.tokenOutMins) {
      Coin.encode(v!, writer.uint32(50).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgExitPool {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgExitPool } as MsgExitPool;
    message.tokenOutMins = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.connectionId = reader.string();
          break;
        case 3:
          message.timeoutTimestamp = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.poolId = longToNumber(reader.uint64() as Long);
          break;
        case 5:
          message.shareInAmount = reader.string();
          break;
        case 6:
          message.tokenOutMins.push(Coin.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgExitPool {
    const message = { ...baseMsgExitPool } as MsgExitPool;
    message.tokenOutMins = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = String(object.connectionId);
    } else {
      message.connectionId = "";
    }
    if (
      object.timeoutTimestamp !== undefined &&
      object.timeoutTimestamp !== null
    ) {
      message.timeoutTimestamp = Number(object.timeoutTimestamp);
    } else {
      message.timeoutTimestamp = 0;
    }
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

  toJSON(message: MsgExitPool): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.connectionId !== undefined &&
      (obj.connectionId = message.connectionId);
    message.timeoutTimestamp !== undefined &&
      (obj.timeoutTimestamp = message.timeoutTimestamp);
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

  fromPartial(object: DeepPartial<MsgExitPool>): MsgExitPool {
    const message = { ...baseMsgExitPool } as MsgExitPool;
    message.tokenOutMins = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.connectionId !== undefined && object.connectionId !== null) {
      message.connectionId = object.connectionId;
    } else {
      message.connectionId = "";
    }
    if (
      object.timeoutTimestamp !== undefined &&
      object.timeoutTimestamp !== null
    ) {
      message.timeoutTimestamp = object.timeoutTimestamp;
    } else {
      message.timeoutTimestamp = 0;
    }
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

const baseMsgExitPoolResponse: object = {};

export const MsgExitPoolResponse = {
  encode(_: MsgExitPoolResponse, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgExitPoolResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgExitPoolResponse } as MsgExitPoolResponse;
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

  fromJSON(_: any): MsgExitPoolResponse {
    const message = { ...baseMsgExitPoolResponse } as MsgExitPoolResponse;
    return message;
  },

  toJSON(_: MsgExitPoolResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<MsgExitPoolResponse>): MsgExitPoolResponse {
    const message = { ...baseMsgExitPoolResponse } as MsgExitPoolResponse;
    return message;
  },
};

const baseMsgIbcTransfer: object = { creator: "" };

export const MsgIbcTransfer = {
  encode(message: MsgIbcTransfer, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgIbcTransfer {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgIbcTransfer } as MsgIbcTransfer;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgIbcTransfer {
    const message = { ...baseMsgIbcTransfer } as MsgIbcTransfer;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    return message;
  },

  toJSON(message: MsgIbcTransfer): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgIbcTransfer>): MsgIbcTransfer {
    const message = { ...baseMsgIbcTransfer } as MsgIbcTransfer;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    return message;
  },
};

const baseMsgIbcTransferResponse: object = {};

export const MsgIbcTransferResponse = {
  encode(_: MsgIbcTransferResponse, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgIbcTransferResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgIbcTransferResponse } as MsgIbcTransferResponse;
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

  fromJSON(_: any): MsgIbcTransferResponse {
    const message = { ...baseMsgIbcTransferResponse } as MsgIbcTransferResponse;
    return message;
  },

  toJSON(_: MsgIbcTransferResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<MsgIbcTransferResponse>): MsgIbcTransferResponse {
    const message = { ...baseMsgIbcTransferResponse } as MsgIbcTransferResponse;
    return message;
  },
};

/** Msg defines the Msg service. */
export interface Msg {
  RegisterAccount(
    request: MsgRegisterAccount
  ): Promise<MsgRegisterAccountResponse>;
  CreatePool(request: MsgCreatePool): Promise<MsgCreatePoolResponse>;
  JoinPool(request: MsgJoinPool): Promise<MsgJoinPoolResponse>;
  ExitPool(request: MsgExitPool): Promise<MsgExitPoolResponse>;
  /** this line is used by starport scaffolding # proto/tx/rpc */
  IbcTransfer(request: MsgIbcTransfer): Promise<MsgIbcTransferResponse>;
}

export class MsgClientImpl implements Msg {
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
  }
  RegisterAccount(
    request: MsgRegisterAccount
  ): Promise<MsgRegisterAccountResponse> {
    const data = MsgRegisterAccount.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.intergamm.Msg",
      "RegisterAccount",
      data
    );
    return promise.then((data) =>
      MsgRegisterAccountResponse.decode(new Reader(data))
    );
  }

  CreatePool(request: MsgCreatePool): Promise<MsgCreatePoolResponse> {
    const data = MsgCreatePool.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.intergamm.Msg",
      "CreatePool",
      data
    );
    return promise.then((data) =>
      MsgCreatePoolResponse.decode(new Reader(data))
    );
  }

  JoinPool(request: MsgJoinPool): Promise<MsgJoinPoolResponse> {
    const data = MsgJoinPool.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.intergamm.Msg",
      "JoinPool",
      data
    );
    return promise.then((data) => MsgJoinPoolResponse.decode(new Reader(data)));
  }

  ExitPool(request: MsgExitPool): Promise<MsgExitPoolResponse> {
    const data = MsgExitPool.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.intergamm.Msg",
      "ExitPool",
      data
    );
    return promise.then((data) => MsgExitPoolResponse.decode(new Reader(data)));
  }

  IbcTransfer(request: MsgIbcTransfer): Promise<MsgIbcTransferResponse> {
    const data = MsgIbcTransfer.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.intergamm.Msg",
      "IbcTransfer",
      data
    );
    return promise.then((data) =>
      MsgIbcTransferResponse.decode(new Reader(data))
    );
  }
}

interface Rpc {
  request(
    service: string,
    method: string,
    data: Uint8Array
  ): Promise<Uint8Array>;
}

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
