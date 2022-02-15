/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";

export const protobufPackage = "abag.quasarnode.qoracle";

export interface MsgCreatePoolPosition {
  creator: string;
  poolID: number;
  aPY: number;
  tVL: number;
  lastUpdatedTime: number;
}

export interface MsgCreatePoolPositionResponse {}

export interface MsgUpdatePoolPosition {
  creator: string;
  poolID: number;
  aPY: number;
  tVL: number;
  lastUpdatedTime: number;
}

export interface MsgUpdatePoolPositionResponse {}

export interface MsgDeletePoolPosition {
  creator: string;
  poolID: number;
}

export interface MsgDeletePoolPositionResponse {}

const baseMsgCreatePoolPosition: object = {
  creator: "",
  poolID: 0,
  aPY: 0,
  tVL: 0,
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
    if (message.poolID !== 0) {
      writer.uint32(16).uint64(message.poolID);
    }
    if (message.aPY !== 0) {
      writer.uint32(24).uint64(message.aPY);
    }
    if (message.tVL !== 0) {
      writer.uint32(32).uint64(message.tVL);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(40).uint64(message.lastUpdatedTime);
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
          message.poolID = longToNumber(reader.uint64() as Long);
          break;
        case 3:
          message.aPY = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.tVL = longToNumber(reader.uint64() as Long);
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

  fromJSON(object: any): MsgCreatePoolPosition {
    const message = { ...baseMsgCreatePoolPosition } as MsgCreatePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolID !== undefined && object.poolID !== null) {
      message.poolID = Number(object.poolID);
    } else {
      message.poolID = 0;
    }
    if (object.aPY !== undefined && object.aPY !== null) {
      message.aPY = Number(object.aPY);
    } else {
      message.aPY = 0;
    }
    if (object.tVL !== undefined && object.tVL !== null) {
      message.tVL = Number(object.tVL);
    } else {
      message.tVL = 0;
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
    message.poolID !== undefined && (obj.poolID = message.poolID);
    message.aPY !== undefined && (obj.aPY = message.aPY);
    message.tVL !== undefined && (obj.tVL = message.tVL);
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
    if (object.poolID !== undefined && object.poolID !== null) {
      message.poolID = object.poolID;
    } else {
      message.poolID = 0;
    }
    if (object.aPY !== undefined && object.aPY !== null) {
      message.aPY = object.aPY;
    } else {
      message.aPY = 0;
    }
    if (object.tVL !== undefined && object.tVL !== null) {
      message.tVL = object.tVL;
    } else {
      message.tVL = 0;
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
  poolID: 0,
  aPY: 0,
  tVL: 0,
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
    if (message.poolID !== 0) {
      writer.uint32(16).uint64(message.poolID);
    }
    if (message.aPY !== 0) {
      writer.uint32(24).uint64(message.aPY);
    }
    if (message.tVL !== 0) {
      writer.uint32(32).uint64(message.tVL);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(40).uint64(message.lastUpdatedTime);
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
          message.poolID = longToNumber(reader.uint64() as Long);
          break;
        case 3:
          message.aPY = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.tVL = longToNumber(reader.uint64() as Long);
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

  fromJSON(object: any): MsgUpdatePoolPosition {
    const message = { ...baseMsgUpdatePoolPosition } as MsgUpdatePoolPosition;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.poolID !== undefined && object.poolID !== null) {
      message.poolID = Number(object.poolID);
    } else {
      message.poolID = 0;
    }
    if (object.aPY !== undefined && object.aPY !== null) {
      message.aPY = Number(object.aPY);
    } else {
      message.aPY = 0;
    }
    if (object.tVL !== undefined && object.tVL !== null) {
      message.tVL = Number(object.tVL);
    } else {
      message.tVL = 0;
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
    message.poolID !== undefined && (obj.poolID = message.poolID);
    message.aPY !== undefined && (obj.aPY = message.aPY);
    message.tVL !== undefined && (obj.tVL = message.tVL);
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
    if (object.poolID !== undefined && object.poolID !== null) {
      message.poolID = object.poolID;
    } else {
      message.poolID = 0;
    }
    if (object.aPY !== undefined && object.aPY !== null) {
      message.aPY = object.aPY;
    } else {
      message.aPY = 0;
    }
    if (object.tVL !== undefined && object.tVL !== null) {
      message.tVL = object.tVL;
    } else {
      message.tVL = 0;
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

const baseMsgDeletePoolPosition: object = { creator: "", poolID: 0 };

export const MsgDeletePoolPosition = {
  encode(
    message: MsgDeletePoolPosition,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.poolID !== 0) {
      writer.uint32(16).uint64(message.poolID);
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
          message.poolID = longToNumber(reader.uint64() as Long);
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
    if (object.poolID !== undefined && object.poolID !== null) {
      message.poolID = Number(object.poolID);
    } else {
      message.poolID = 0;
    }
    return message;
  },

  toJSON(message: MsgDeletePoolPosition): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.poolID !== undefined && (obj.poolID = message.poolID);
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
    if (object.poolID !== undefined && object.poolID !== null) {
      message.poolID = object.poolID;
    } else {
      message.poolID = 0;
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

/** Msg defines the Msg service. */
export interface Msg {
  CreatePoolPosition(
    request: MsgCreatePoolPosition
  ): Promise<MsgCreatePoolPositionResponse>;
  UpdatePoolPosition(
    request: MsgUpdatePoolPosition
  ): Promise<MsgUpdatePoolPositionResponse>;
  /** this line is used by starport scaffolding # proto/tx/rpc */
  DeletePoolPosition(
    request: MsgDeletePoolPosition
  ): Promise<MsgDeletePoolPositionResponse>;
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
