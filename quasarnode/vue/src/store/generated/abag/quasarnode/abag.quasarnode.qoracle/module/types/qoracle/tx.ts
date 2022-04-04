/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { PoolMetrics } from "../qoracle/pool_metrics";
import { BalancerPool } from "../osmosis/gamm/pool-models/balancer/balancerPool";

export const protobufPackage = "abag.quasarnode.qoracle";

export interface MsgCreatePoolPosition {
  creator: string;
  poolId: string;
  metrics: PoolMetrics | undefined;
  lastUpdatedTime: number;
}

export interface MsgCreatePoolPositionResponse {}

export interface MsgUpdatePoolPosition {
  creator: string;
  poolId: string;
  metrics: PoolMetrics | undefined;
  lastUpdatedTime: number;
}

export interface MsgUpdatePoolPositionResponse {}

export interface MsgDeletePoolPosition {
  creator: string;
  poolId: string;
}

export interface MsgDeletePoolPositionResponse {}

export interface MsgCreatePoolRanking {
  creator: string;
  poolIdsSortedByAPY: string[];
  poolIdsSortedByTVL: string[];
  lastUpdatedTime: number;
}

export interface MsgCreatePoolRankingResponse {}

export interface MsgUpdatePoolRanking {
  creator: string;
  poolIdsSortedByAPY: string[];
  poolIdsSortedByTVL: string[];
  lastUpdatedTime: number;
}

export interface MsgUpdatePoolRankingResponse {}

export interface MsgDeletePoolRanking {
  creator: string;
}

export interface MsgDeletePoolRankingResponse {}

export interface MsgCreatePoolSpotPrice {
  creator: string;
  poolId: string;
  denomIn: string;
  denomOut: string;
  price: string;
  lastUpdatedTime: number;
}

export interface MsgCreatePoolSpotPriceResponse {}

export interface MsgUpdatePoolSpotPrice {
  creator: string;
  poolId: string;
  denomIn: string;
  denomOut: string;
  price: string;
  lastUpdatedTime: number;
}

export interface MsgUpdatePoolSpotPriceResponse {}

export interface MsgDeletePoolSpotPrice {
  creator: string;
  poolId: string;
  denomIn: string;
  denomOut: string;
}

export interface MsgDeletePoolSpotPriceResponse {}

export interface MsgCreatePoolInfo {
  creator: string;
  poolId: string;
  info: BalancerPool | undefined;
  lastUpdatedTime: number;
}

export interface MsgCreatePoolInfoResponse {}

export interface MsgUpdatePoolInfo {
  creator: string;
  poolId: string;
  info: BalancerPool | undefined;
  lastUpdatedTime: number;
}

export interface MsgUpdatePoolInfoResponse {}

export interface MsgDeletePoolInfo {
  creator: string;
  poolId: string;
}

export interface MsgDeletePoolInfoResponse {}

const baseMsgCreatePoolPosition: object = {
  creator: "",
  poolId: "",
  lastUpdatedTime: 0,
};

export const MsgCreatePoolPosition = {
  encode(
    message: MsgCreatePoolPosition,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    if (message.metrics !== undefined) {
      PoolMetrics.encode(message.metrics, writer.uint32(26).fork()).ldelim();
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(32).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolPosition {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgCreatePoolPosition } as MsgCreatePoolPosition;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        case 3:
          message.metrics = PoolMetrics.decode(reader, reader.uint32());
          break;
        case 4:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgCreatePoolPosition {
    const message = { ...baseMsgCreatePoolPosition } as MsgCreatePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.metrics !== undefined && object.metrics !== null) {
      message.metrics = PoolMetrics.fromJSON(object.metrics);
    } else {
      message.metrics = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgCreatePoolPosition): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.metrics !== undefined &&
      (obj.metrics = message.metrics
        ? PoolMetrics.toJSON(message.metrics)
        : undefined);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(
    object: DeepPartial<MsgCreatePoolPosition>
  ): MsgCreatePoolPosition {
    const message = { ...baseMsgCreatePoolPosition } as MsgCreatePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.metrics !== undefined && object.metrics !== null) {
      message.metrics = PoolMetrics.fromPartial(object.metrics);
    } else {
      message.metrics = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgCreatePoolPositionResponse: object = {};

export const MsgCreatePoolPositionResponse = {
  encode(
    _: MsgCreatePoolPositionResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgCreatePoolPositionResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgCreatePoolPositionResponse,
    } as MsgCreatePoolPositionResponse;
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

  fromJSON(_: any): MsgCreatePoolPositionResponse {
    const message = {
      ...baseMsgCreatePoolPositionResponse,
    } as MsgCreatePoolPositionResponse;
    return message;
  },

  toJSON(_: MsgCreatePoolPositionResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgCreatePoolPositionResponse>
  ): MsgCreatePoolPositionResponse {
    const message = {
      ...baseMsgCreatePoolPositionResponse,
    } as MsgCreatePoolPositionResponse;
    return message;
  },
};

const baseMsgUpdatePoolPosition: object = {
  creator: "",
  poolId: "",
  lastUpdatedTime: 0,
};

export const MsgUpdatePoolPosition = {
  encode(
    message: MsgUpdatePoolPosition,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    if (message.metrics !== undefined) {
      PoolMetrics.encode(message.metrics, writer.uint32(26).fork()).ldelim();
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(32).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolPosition {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgUpdatePoolPosition } as MsgUpdatePoolPosition;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        case 3:
          message.metrics = PoolMetrics.decode(reader, reader.uint32());
          break;
        case 4:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgUpdatePoolPosition {
    const message = { ...baseMsgUpdatePoolPosition } as MsgUpdatePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.metrics !== undefined && object.metrics !== null) {
      message.metrics = PoolMetrics.fromJSON(object.metrics);
    } else {
      message.metrics = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgUpdatePoolPosition): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.metrics !== undefined &&
      (obj.metrics = message.metrics
        ? PoolMetrics.toJSON(message.metrics)
        : undefined);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(
    object: DeepPartial<MsgUpdatePoolPosition>
  ): MsgUpdatePoolPosition {
    const message = { ...baseMsgUpdatePoolPosition } as MsgUpdatePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.metrics !== undefined && object.metrics !== null) {
      message.metrics = PoolMetrics.fromPartial(object.metrics);
    } else {
      message.metrics = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgUpdatePoolPositionResponse: object = {};

export const MsgUpdatePoolPositionResponse = {
  encode(
    _: MsgUpdatePoolPositionResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgUpdatePoolPositionResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgUpdatePoolPositionResponse,
    } as MsgUpdatePoolPositionResponse;
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

  fromJSON(_: any): MsgUpdatePoolPositionResponse {
    const message = {
      ...baseMsgUpdatePoolPositionResponse,
    } as MsgUpdatePoolPositionResponse;
    return message;
  },

  toJSON(_: MsgUpdatePoolPositionResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgUpdatePoolPositionResponse>
  ): MsgUpdatePoolPositionResponse {
    const message = {
      ...baseMsgUpdatePoolPositionResponse,
    } as MsgUpdatePoolPositionResponse;
    return message;
  },
};

const baseMsgDeletePoolPosition: object = { creator: "", poolId: "" };

export const MsgDeletePoolPosition = {
  encode(
    message: MsgDeletePoolPosition,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolPosition {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgDeletePoolPosition } as MsgDeletePoolPosition;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgDeletePoolPosition {
    const message = { ...baseMsgDeletePoolPosition } as MsgDeletePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    return message;
  },

  toJSON(message: MsgDeletePoolPosition): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    return obj;
  },

  fromPartial(
    object: DeepPartial<MsgDeletePoolPosition>
  ): MsgDeletePoolPosition {
    const message = { ...baseMsgDeletePoolPosition } as MsgDeletePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    return message;
  },
};

const baseMsgDeletePoolPositionResponse: object = {};

export const MsgDeletePoolPositionResponse = {
  encode(
    _: MsgDeletePoolPositionResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgDeletePoolPositionResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgDeletePoolPositionResponse,
    } as MsgDeletePoolPositionResponse;
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

  fromJSON(_: any): MsgDeletePoolPositionResponse {
    const message = {
      ...baseMsgDeletePoolPositionResponse,
    } as MsgDeletePoolPositionResponse;
    return message;
  },

  toJSON(_: MsgDeletePoolPositionResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgDeletePoolPositionResponse>
  ): MsgDeletePoolPositionResponse {
    const message = {
      ...baseMsgDeletePoolPositionResponse,
    } as MsgDeletePoolPositionResponse;
    return message;
  },
};

const baseMsgCreatePoolRanking: object = {
  creator: "",
  poolIdsSortedByAPY: "",
  poolIdsSortedByTVL: "",
  lastUpdatedTime: 0,
};

export const MsgCreatePoolRanking = {
  encode(
    message: MsgCreatePoolRanking,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    for (const v of message.poolIdsSortedByAPY) {
      writer.uint32(26).string(v!);
    }
    for (const v of message.poolIdsSortedByTVL) {
      writer.uint32(34).string(v!);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(40).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolRanking {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgCreatePoolRanking } as MsgCreatePoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 3:
          message.poolIdsSortedByAPY.push(reader.string());
          break;
        case 4:
          message.poolIdsSortedByTVL.push(reader.string());
          break;
        case 5:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgCreatePoolRanking {
    const message = { ...baseMsgCreatePoolRanking } as MsgCreatePoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (
      object.poolIdsSortedByAPY !== undefined &&
      object.poolIdsSortedByAPY !== null
    ) {
      for (const e of object.poolIdsSortedByAPY) {
        message.poolIdsSortedByAPY.push(String(e));
      }
    }
    if (
      object.poolIdsSortedByTVL !== undefined &&
      object.poolIdsSortedByTVL !== null
    ) {
      for (const e of object.poolIdsSortedByTVL) {
        message.poolIdsSortedByTVL.push(String(e));
      }
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgCreatePoolRanking): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    if (message.poolIdsSortedByAPY) {
      obj.poolIdsSortedByAPY = message.poolIdsSortedByAPY.map((e) => e);
    } else {
      obj.poolIdsSortedByAPY = [];
    }
    if (message.poolIdsSortedByTVL) {
      obj.poolIdsSortedByTVL = message.poolIdsSortedByTVL.map((e) => e);
    } else {
      obj.poolIdsSortedByTVL = [];
    }
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgCreatePoolRanking>): MsgCreatePoolRanking {
    const message = { ...baseMsgCreatePoolRanking } as MsgCreatePoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (
      object.poolIdsSortedByAPY !== undefined &&
      object.poolIdsSortedByAPY !== null
    ) {
      for (const e of object.poolIdsSortedByAPY) {
        message.poolIdsSortedByAPY.push(e);
      }
    }
    if (
      object.poolIdsSortedByTVL !== undefined &&
      object.poolIdsSortedByTVL !== null
    ) {
      for (const e of object.poolIdsSortedByTVL) {
        message.poolIdsSortedByTVL.push(e);
      }
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgCreatePoolRankingResponse: object = {};

export const MsgCreatePoolRankingResponse = {
  encode(
    _: MsgCreatePoolRankingResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgCreatePoolRankingResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgCreatePoolRankingResponse,
    } as MsgCreatePoolRankingResponse;
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

  fromJSON(_: any): MsgCreatePoolRankingResponse {
    const message = {
      ...baseMsgCreatePoolRankingResponse,
    } as MsgCreatePoolRankingResponse;
    return message;
  },

  toJSON(_: MsgCreatePoolRankingResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgCreatePoolRankingResponse>
  ): MsgCreatePoolRankingResponse {
    const message = {
      ...baseMsgCreatePoolRankingResponse,
    } as MsgCreatePoolRankingResponse;
    return message;
  },
};

const baseMsgUpdatePoolRanking: object = {
  creator: "",
  poolIdsSortedByAPY: "",
  poolIdsSortedByTVL: "",
  lastUpdatedTime: 0,
};

export const MsgUpdatePoolRanking = {
  encode(
    message: MsgUpdatePoolRanking,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    for (const v of message.poolIdsSortedByAPY) {
      writer.uint32(26).string(v!);
    }
    for (const v of message.poolIdsSortedByTVL) {
      writer.uint32(34).string(v!);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(40).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolRanking {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgUpdatePoolRanking } as MsgUpdatePoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 3:
          message.poolIdsSortedByAPY.push(reader.string());
          break;
        case 4:
          message.poolIdsSortedByTVL.push(reader.string());
          break;
        case 5:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgUpdatePoolRanking {
    const message = { ...baseMsgUpdatePoolRanking } as MsgUpdatePoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (
      object.poolIdsSortedByAPY !== undefined &&
      object.poolIdsSortedByAPY !== null
    ) {
      for (const e of object.poolIdsSortedByAPY) {
        message.poolIdsSortedByAPY.push(String(e));
      }
    }
    if (
      object.poolIdsSortedByTVL !== undefined &&
      object.poolIdsSortedByTVL !== null
    ) {
      for (const e of object.poolIdsSortedByTVL) {
        message.poolIdsSortedByTVL.push(String(e));
      }
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgUpdatePoolRanking): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    if (message.poolIdsSortedByAPY) {
      obj.poolIdsSortedByAPY = message.poolIdsSortedByAPY.map((e) => e);
    } else {
      obj.poolIdsSortedByAPY = [];
    }
    if (message.poolIdsSortedByTVL) {
      obj.poolIdsSortedByTVL = message.poolIdsSortedByTVL.map((e) => e);
    } else {
      obj.poolIdsSortedByTVL = [];
    }
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgUpdatePoolRanking>): MsgUpdatePoolRanking {
    const message = { ...baseMsgUpdatePoolRanking } as MsgUpdatePoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (
      object.poolIdsSortedByAPY !== undefined &&
      object.poolIdsSortedByAPY !== null
    ) {
      for (const e of object.poolIdsSortedByAPY) {
        message.poolIdsSortedByAPY.push(e);
      }
    }
    if (
      object.poolIdsSortedByTVL !== undefined &&
      object.poolIdsSortedByTVL !== null
    ) {
      for (const e of object.poolIdsSortedByTVL) {
        message.poolIdsSortedByTVL.push(e);
      }
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgUpdatePoolRankingResponse: object = {};

export const MsgUpdatePoolRankingResponse = {
  encode(
    _: MsgUpdatePoolRankingResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgUpdatePoolRankingResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgUpdatePoolRankingResponse,
    } as MsgUpdatePoolRankingResponse;
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

  fromJSON(_: any): MsgUpdatePoolRankingResponse {
    const message = {
      ...baseMsgUpdatePoolRankingResponse,
    } as MsgUpdatePoolRankingResponse;
    return message;
  },

  toJSON(_: MsgUpdatePoolRankingResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgUpdatePoolRankingResponse>
  ): MsgUpdatePoolRankingResponse {
    const message = {
      ...baseMsgUpdatePoolRankingResponse,
    } as MsgUpdatePoolRankingResponse;
    return message;
  },
};

const baseMsgDeletePoolRanking: object = { creator: "" };

export const MsgDeletePoolRanking = {
  encode(
    message: MsgDeletePoolRanking,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolRanking {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgDeletePoolRanking } as MsgDeletePoolRanking;
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

  fromJSON(object: any): MsgDeletePoolRanking {
    const message = { ...baseMsgDeletePoolRanking } as MsgDeletePoolRanking;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    return message;
  },

  toJSON(message: MsgDeletePoolRanking): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgDeletePoolRanking>): MsgDeletePoolRanking {
    const message = { ...baseMsgDeletePoolRanking } as MsgDeletePoolRanking;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    return message;
  },
};

const baseMsgDeletePoolRankingResponse: object = {};

export const MsgDeletePoolRankingResponse = {
  encode(
    _: MsgDeletePoolRankingResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgDeletePoolRankingResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgDeletePoolRankingResponse,
    } as MsgDeletePoolRankingResponse;
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

  fromJSON(_: any): MsgDeletePoolRankingResponse {
    const message = {
      ...baseMsgDeletePoolRankingResponse,
    } as MsgDeletePoolRankingResponse;
    return message;
  },

  toJSON(_: MsgDeletePoolRankingResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgDeletePoolRankingResponse>
  ): MsgDeletePoolRankingResponse {
    const message = {
      ...baseMsgDeletePoolRankingResponse,
    } as MsgDeletePoolRankingResponse;
    return message;
  },
};

const baseMsgCreatePoolSpotPrice: object = {
  creator: "",
  poolId: "",
  denomIn: "",
  denomOut: "",
  price: "",
  lastUpdatedTime: 0,
};

export const MsgCreatePoolSpotPrice = {
  encode(
    message: MsgCreatePoolSpotPrice,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    if (message.denomIn !== "") {
      writer.uint32(26).string(message.denomIn);
    }
    if (message.denomOut !== "") {
      writer.uint32(34).string(message.denomOut);
    }
    if (message.price !== "") {
      writer.uint32(42).string(message.price);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(48).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolSpotPrice {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgCreatePoolSpotPrice } as MsgCreatePoolSpotPrice;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        case 3:
          message.denomIn = reader.string();
          break;
        case 4:
          message.denomOut = reader.string();
          break;
        case 5:
          message.price = reader.string();
          break;
        case 6:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgCreatePoolSpotPrice {
    const message = { ...baseMsgCreatePoolSpotPrice } as MsgCreatePoolSpotPrice;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = String(object.denomIn);
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = String(object.denomOut);
    } else {
      message.denomOut = "";
    }
    if (object.price !== undefined && object.price !== null) {
      message.price = String(object.price);
    } else {
      message.price = "";
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgCreatePoolSpotPrice): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.denomIn !== undefined && (obj.denomIn = message.denomIn);
    message.denomOut !== undefined && (obj.denomOut = message.denomOut);
    message.price !== undefined && (obj.price = message.price);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(
    object: DeepPartial<MsgCreatePoolSpotPrice>
  ): MsgCreatePoolSpotPrice {
    const message = { ...baseMsgCreatePoolSpotPrice } as MsgCreatePoolSpotPrice;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = object.denomIn;
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = object.denomOut;
    } else {
      message.denomOut = "";
    }
    if (object.price !== undefined && object.price !== null) {
      message.price = object.price;
    } else {
      message.price = "";
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgCreatePoolSpotPriceResponse: object = {};

export const MsgCreatePoolSpotPriceResponse = {
  encode(
    _: MsgCreatePoolSpotPriceResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgCreatePoolSpotPriceResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgCreatePoolSpotPriceResponse,
    } as MsgCreatePoolSpotPriceResponse;
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

  fromJSON(_: any): MsgCreatePoolSpotPriceResponse {
    const message = {
      ...baseMsgCreatePoolSpotPriceResponse,
    } as MsgCreatePoolSpotPriceResponse;
    return message;
  },

  toJSON(_: MsgCreatePoolSpotPriceResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgCreatePoolSpotPriceResponse>
  ): MsgCreatePoolSpotPriceResponse {
    const message = {
      ...baseMsgCreatePoolSpotPriceResponse,
    } as MsgCreatePoolSpotPriceResponse;
    return message;
  },
};

const baseMsgUpdatePoolSpotPrice: object = {
  creator: "",
  poolId: "",
  denomIn: "",
  denomOut: "",
  price: "",
  lastUpdatedTime: 0,
};

export const MsgUpdatePoolSpotPrice = {
  encode(
    message: MsgUpdatePoolSpotPrice,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    if (message.denomIn !== "") {
      writer.uint32(26).string(message.denomIn);
    }
    if (message.denomOut !== "") {
      writer.uint32(34).string(message.denomOut);
    }
    if (message.price !== "") {
      writer.uint32(42).string(message.price);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(48).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolSpotPrice {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgUpdatePoolSpotPrice } as MsgUpdatePoolSpotPrice;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        case 3:
          message.denomIn = reader.string();
          break;
        case 4:
          message.denomOut = reader.string();
          break;
        case 5:
          message.price = reader.string();
          break;
        case 6:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgUpdatePoolSpotPrice {
    const message = { ...baseMsgUpdatePoolSpotPrice } as MsgUpdatePoolSpotPrice;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = String(object.denomIn);
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = String(object.denomOut);
    } else {
      message.denomOut = "";
    }
    if (object.price !== undefined && object.price !== null) {
      message.price = String(object.price);
    } else {
      message.price = "";
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgUpdatePoolSpotPrice): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.denomIn !== undefined && (obj.denomIn = message.denomIn);
    message.denomOut !== undefined && (obj.denomOut = message.denomOut);
    message.price !== undefined && (obj.price = message.price);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(
    object: DeepPartial<MsgUpdatePoolSpotPrice>
  ): MsgUpdatePoolSpotPrice {
    const message = { ...baseMsgUpdatePoolSpotPrice } as MsgUpdatePoolSpotPrice;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = object.denomIn;
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = object.denomOut;
    } else {
      message.denomOut = "";
    }
    if (object.price !== undefined && object.price !== null) {
      message.price = object.price;
    } else {
      message.price = "";
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgUpdatePoolSpotPriceResponse: object = {};

export const MsgUpdatePoolSpotPriceResponse = {
  encode(
    _: MsgUpdatePoolSpotPriceResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgUpdatePoolSpotPriceResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgUpdatePoolSpotPriceResponse,
    } as MsgUpdatePoolSpotPriceResponse;
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

  fromJSON(_: any): MsgUpdatePoolSpotPriceResponse {
    const message = {
      ...baseMsgUpdatePoolSpotPriceResponse,
    } as MsgUpdatePoolSpotPriceResponse;
    return message;
  },

  toJSON(_: MsgUpdatePoolSpotPriceResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgUpdatePoolSpotPriceResponse>
  ): MsgUpdatePoolSpotPriceResponse {
    const message = {
      ...baseMsgUpdatePoolSpotPriceResponse,
    } as MsgUpdatePoolSpotPriceResponse;
    return message;
  },
};

const baseMsgDeletePoolSpotPrice: object = {
  creator: "",
  poolId: "",
  denomIn: "",
  denomOut: "",
};

export const MsgDeletePoolSpotPrice = {
  encode(
    message: MsgDeletePoolSpotPrice,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    if (message.denomIn !== "") {
      writer.uint32(26).string(message.denomIn);
    }
    if (message.denomOut !== "") {
      writer.uint32(34).string(message.denomOut);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolSpotPrice {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgDeletePoolSpotPrice } as MsgDeletePoolSpotPrice;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        case 3:
          message.denomIn = reader.string();
          break;
        case 4:
          message.denomOut = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgDeletePoolSpotPrice {
    const message = { ...baseMsgDeletePoolSpotPrice } as MsgDeletePoolSpotPrice;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = String(object.denomIn);
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = String(object.denomOut);
    } else {
      message.denomOut = "";
    }
    return message;
  },

  toJSON(message: MsgDeletePoolSpotPrice): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.denomIn !== undefined && (obj.denomIn = message.denomIn);
    message.denomOut !== undefined && (obj.denomOut = message.denomOut);
    return obj;
  },

  fromPartial(
    object: DeepPartial<MsgDeletePoolSpotPrice>
  ): MsgDeletePoolSpotPrice {
    const message = { ...baseMsgDeletePoolSpotPrice } as MsgDeletePoolSpotPrice;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = object.denomIn;
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = object.denomOut;
    } else {
      message.denomOut = "";
    }
    return message;
  },
};

const baseMsgDeletePoolSpotPriceResponse: object = {};

export const MsgDeletePoolSpotPriceResponse = {
  encode(
    _: MsgDeletePoolSpotPriceResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgDeletePoolSpotPriceResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgDeletePoolSpotPriceResponse,
    } as MsgDeletePoolSpotPriceResponse;
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

  fromJSON(_: any): MsgDeletePoolSpotPriceResponse {
    const message = {
      ...baseMsgDeletePoolSpotPriceResponse,
    } as MsgDeletePoolSpotPriceResponse;
    return message;
  },

  toJSON(_: MsgDeletePoolSpotPriceResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgDeletePoolSpotPriceResponse>
  ): MsgDeletePoolSpotPriceResponse {
    const message = {
      ...baseMsgDeletePoolSpotPriceResponse,
    } as MsgDeletePoolSpotPriceResponse;
    return message;
  },
};

const baseMsgCreatePoolInfo: object = {
  creator: "",
  poolId: "",
  lastUpdatedTime: 0,
};

export const MsgCreatePoolInfo = {
  encode(message: MsgCreatePoolInfo, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    if (message.info !== undefined) {
      BalancerPool.encode(message.info, writer.uint32(26).fork()).ldelim();
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(32).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgCreatePoolInfo {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgCreatePoolInfo } as MsgCreatePoolInfo;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        case 3:
          message.info = BalancerPool.decode(reader, reader.uint32());
          break;
        case 4:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgCreatePoolInfo {
    const message = { ...baseMsgCreatePoolInfo } as MsgCreatePoolInfo;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.info !== undefined && object.info !== null) {
      message.info = BalancerPool.fromJSON(object.info);
    } else {
      message.info = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgCreatePoolInfo): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.info !== undefined &&
      (obj.info = message.info ? BalancerPool.toJSON(message.info) : undefined);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgCreatePoolInfo>): MsgCreatePoolInfo {
    const message = { ...baseMsgCreatePoolInfo } as MsgCreatePoolInfo;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.info !== undefined && object.info !== null) {
      message.info = BalancerPool.fromPartial(object.info);
    } else {
      message.info = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgCreatePoolInfoResponse: object = {};

export const MsgCreatePoolInfoResponse = {
  encode(
    _: MsgCreatePoolInfoResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgCreatePoolInfoResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgCreatePoolInfoResponse,
    } as MsgCreatePoolInfoResponse;
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

  fromJSON(_: any): MsgCreatePoolInfoResponse {
    const message = {
      ...baseMsgCreatePoolInfoResponse,
    } as MsgCreatePoolInfoResponse;
    return message;
  },

  toJSON(_: MsgCreatePoolInfoResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgCreatePoolInfoResponse>
  ): MsgCreatePoolInfoResponse {
    const message = {
      ...baseMsgCreatePoolInfoResponse,
    } as MsgCreatePoolInfoResponse;
    return message;
  },
};

const baseMsgUpdatePoolInfo: object = {
  creator: "",
  poolId: "",
  lastUpdatedTime: 0,
};

export const MsgUpdatePoolInfo = {
  encode(message: MsgUpdatePoolInfo, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    if (message.info !== undefined) {
      BalancerPool.encode(message.info, writer.uint32(26).fork()).ldelim();
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(32).uint64(message.lastUpdatedTime);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgUpdatePoolInfo {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgUpdatePoolInfo } as MsgUpdatePoolInfo;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        case 3:
          message.info = BalancerPool.decode(reader, reader.uint32());
          break;
        case 4:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgUpdatePoolInfo {
    const message = { ...baseMsgUpdatePoolInfo } as MsgUpdatePoolInfo;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.info !== undefined && object.info !== null) {
      message.info = BalancerPool.fromJSON(object.info);
    } else {
      message.info = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = Number(object.lastUpdatedTime);
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },

  toJSON(message: MsgUpdatePoolInfo): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.info !== undefined &&
      (obj.info = message.info ? BalancerPool.toJSON(message.info) : undefined);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgUpdatePoolInfo>): MsgUpdatePoolInfo {
    const message = { ...baseMsgUpdatePoolInfo } as MsgUpdatePoolInfo;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.info !== undefined && object.info !== null) {
      message.info = BalancerPool.fromPartial(object.info);
    } else {
      message.info = undefined;
    }
    if (
      object.lastUpdatedTime !== undefined &&
      object.lastUpdatedTime !== null
    ) {
      message.lastUpdatedTime = object.lastUpdatedTime;
    } else {
      message.lastUpdatedTime = 0;
    }
    return message;
  },
};

const baseMsgUpdatePoolInfoResponse: object = {};

export const MsgUpdatePoolInfoResponse = {
  encode(
    _: MsgUpdatePoolInfoResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgUpdatePoolInfoResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgUpdatePoolInfoResponse,
    } as MsgUpdatePoolInfoResponse;
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

  fromJSON(_: any): MsgUpdatePoolInfoResponse {
    const message = {
      ...baseMsgUpdatePoolInfoResponse,
    } as MsgUpdatePoolInfoResponse;
    return message;
  },

  toJSON(_: MsgUpdatePoolInfoResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgUpdatePoolInfoResponse>
  ): MsgUpdatePoolInfoResponse {
    const message = {
      ...baseMsgUpdatePoolInfoResponse,
    } as MsgUpdatePoolInfoResponse;
    return message;
  },
};

const baseMsgDeletePoolInfo: object = { creator: "", poolId: "" };

export const MsgDeletePoolInfo = {
  encode(message: MsgDeletePoolInfo, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolId !== "") {
      writer.uint32(18).string(message.poolId);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgDeletePoolInfo {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgDeletePoolInfo } as MsgDeletePoolInfo;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.poolId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgDeletePoolInfo {
    const message = { ...baseMsgDeletePoolInfo } as MsgDeletePoolInfo;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    return message;
  },

  toJSON(message: MsgDeletePoolInfo): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolId !== undefined && (obj.poolId = message.poolId);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgDeletePoolInfo>): MsgDeletePoolInfo {
    const message = { ...baseMsgDeletePoolInfo } as MsgDeletePoolInfo;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    return message;
  },
};

const baseMsgDeletePoolInfoResponse: object = {};

export const MsgDeletePoolInfoResponse = {
  encode(
    _: MsgDeletePoolInfoResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgDeletePoolInfoResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgDeletePoolInfoResponse,
    } as MsgDeletePoolInfoResponse;
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

  fromJSON(_: any): MsgDeletePoolInfoResponse {
    const message = {
      ...baseMsgDeletePoolInfoResponse,
    } as MsgDeletePoolInfoResponse;
    return message;
  },

  toJSON(_: MsgDeletePoolInfoResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgDeletePoolInfoResponse>
  ): MsgDeletePoolInfoResponse {
    const message = {
      ...baseMsgDeletePoolInfoResponse,
    } as MsgDeletePoolInfoResponse;
    return message;
  },
};

/** Msg defines the Msg service. */
export interface Msg {
  CreatePoolPosition(
    request: MsgCreatePoolPosition
  ): Promise<MsgCreatePoolPositionResponse>;
  UpdatePoolPosition(
    request: MsgUpdatePoolPosition
  ): Promise<MsgUpdatePoolPositionResponse>;
  DeletePoolPosition(
    request: MsgDeletePoolPosition
  ): Promise<MsgDeletePoolPositionResponse>;
  CreatePoolRanking(
    request: MsgCreatePoolRanking
  ): Promise<MsgCreatePoolRankingResponse>;
  UpdatePoolRanking(
    request: MsgUpdatePoolRanking
  ): Promise<MsgUpdatePoolRankingResponse>;
  DeletePoolRanking(
    request: MsgDeletePoolRanking
  ): Promise<MsgDeletePoolRankingResponse>;
  CreatePoolSpotPrice(
    request: MsgCreatePoolSpotPrice
  ): Promise<MsgCreatePoolSpotPriceResponse>;
  UpdatePoolSpotPrice(
    request: MsgUpdatePoolSpotPrice
  ): Promise<MsgUpdatePoolSpotPriceResponse>;
  DeletePoolSpotPrice(
    request: MsgDeletePoolSpotPrice
  ): Promise<MsgDeletePoolSpotPriceResponse>;
  CreatePoolInfo(
    request: MsgCreatePoolInfo
  ): Promise<MsgCreatePoolInfoResponse>;
  UpdatePoolInfo(
    request: MsgUpdatePoolInfo
  ): Promise<MsgUpdatePoolInfoResponse>;
  /** this line is used by starport scaffolding # proto/tx/rpc */
  DeletePoolInfo(
    request: MsgDeletePoolInfo
  ): Promise<MsgDeletePoolInfoResponse>;
}

export class MsgClientImpl implements Msg {
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
  }
  CreatePoolPosition(
    request: MsgCreatePoolPosition
  ): Promise<MsgCreatePoolPositionResponse> {
    const data = MsgCreatePoolPosition.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "CreatePoolPosition",
      data
    );
    return promise.then((data) =>
      MsgCreatePoolPositionResponse.decode(new Reader(data))
    );
  }

  UpdatePoolPosition(
    request: MsgUpdatePoolPosition
  ): Promise<MsgUpdatePoolPositionResponse> {
    const data = MsgUpdatePoolPosition.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "UpdatePoolPosition",
      data
    );
    return promise.then((data) =>
      MsgUpdatePoolPositionResponse.decode(new Reader(data))
    );
  }

  DeletePoolPosition(
    request: MsgDeletePoolPosition
  ): Promise<MsgDeletePoolPositionResponse> {
    const data = MsgDeletePoolPosition.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "DeletePoolPosition",
      data
    );
    return promise.then((data) =>
      MsgDeletePoolPositionResponse.decode(new Reader(data))
    );
  }

  CreatePoolRanking(
    request: MsgCreatePoolRanking
  ): Promise<MsgCreatePoolRankingResponse> {
    const data = MsgCreatePoolRanking.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "CreatePoolRanking",
      data
    );
    return promise.then((data) =>
      MsgCreatePoolRankingResponse.decode(new Reader(data))
    );
  }

  UpdatePoolRanking(
    request: MsgUpdatePoolRanking
  ): Promise<MsgUpdatePoolRankingResponse> {
    const data = MsgUpdatePoolRanking.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "UpdatePoolRanking",
      data
    );
    return promise.then((data) =>
      MsgUpdatePoolRankingResponse.decode(new Reader(data))
    );
  }

  DeletePoolRanking(
    request: MsgDeletePoolRanking
  ): Promise<MsgDeletePoolRankingResponse> {
    const data = MsgDeletePoolRanking.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "DeletePoolRanking",
      data
    );
    return promise.then((data) =>
      MsgDeletePoolRankingResponse.decode(new Reader(data))
    );
  }

  CreatePoolSpotPrice(
    request: MsgCreatePoolSpotPrice
  ): Promise<MsgCreatePoolSpotPriceResponse> {
    const data = MsgCreatePoolSpotPrice.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "CreatePoolSpotPrice",
      data
    );
    return promise.then((data) =>
      MsgCreatePoolSpotPriceResponse.decode(new Reader(data))
    );
  }

  UpdatePoolSpotPrice(
    request: MsgUpdatePoolSpotPrice
  ): Promise<MsgUpdatePoolSpotPriceResponse> {
    const data = MsgUpdatePoolSpotPrice.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "UpdatePoolSpotPrice",
      data
    );
    return promise.then((data) =>
      MsgUpdatePoolSpotPriceResponse.decode(new Reader(data))
    );
  }

  DeletePoolSpotPrice(
    request: MsgDeletePoolSpotPrice
  ): Promise<MsgDeletePoolSpotPriceResponse> {
    const data = MsgDeletePoolSpotPrice.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "DeletePoolSpotPrice",
      data
    );
    return promise.then((data) =>
      MsgDeletePoolSpotPriceResponse.decode(new Reader(data))
    );
  }

  CreatePoolInfo(
    request: MsgCreatePoolInfo
  ): Promise<MsgCreatePoolInfoResponse> {
    const data = MsgCreatePoolInfo.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "CreatePoolInfo",
      data
    );
    return promise.then((data) =>
      MsgCreatePoolInfoResponse.decode(new Reader(data))
    );
  }

  UpdatePoolInfo(
    request: MsgUpdatePoolInfo
  ): Promise<MsgUpdatePoolInfoResponse> {
    const data = MsgUpdatePoolInfo.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "UpdatePoolInfo",
      data
    );
    return promise.then((data) =>
      MsgUpdatePoolInfoResponse.decode(new Reader(data))
    );
  }

  DeletePoolInfo(
    request: MsgDeletePoolInfo
  ): Promise<MsgDeletePoolInfoResponse> {
    const data = MsgDeletePoolInfo.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Msg",
      "DeletePoolInfo",
      data
    );
    return promise.then((data) =>
      MsgDeletePoolInfoResponse.decode(new Reader(data))
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
