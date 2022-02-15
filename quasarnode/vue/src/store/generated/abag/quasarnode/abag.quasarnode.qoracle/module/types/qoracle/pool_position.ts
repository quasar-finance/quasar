/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qoracle";

export interface PoolPosition {
  aPY: number;
  tVL: number;
  lastUpdatedTime: number;
  creator: string;
}

const basePoolPosition: object = {
  aPY: 0,
  tVL: 0,
  lastUpdatedTime: 0,
  creator: "",
};

export const PoolPosition = {
  encode(message: PoolPosition, writer: Writer = Writer.create()): Writer {
    if (message.aPY !== 0) {
      writer.uint32(8).uint64(message.aPY);
    }
    if (message.tVL !== 0) {
      writer.uint32(16).uint64(message.tVL);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(24).uint64(message.lastUpdatedTime);
    }
    if (message.creator !== "") {
      writer.uint32(34).string(message.creator);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): PoolPosition {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...basePoolPosition } as PoolPosition;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.aPY = longToNumber(reader.uint64() as Long);
          break;
        case 2:
          message.tVL = longToNumber(reader.uint64() as Long);
          break;
        case 3:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.creator = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PoolPosition {
    const message = { ...basePoolPosition } as PoolPosition;
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
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    return message;
  },

  toJSON(message: PoolPosition): unknown {
    const obj: any = {};
    message.aPY !== undefined && (obj.aPY = message.aPY);
    message.tVL !== undefined && (obj.tVL = message.tVL);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    message.creator !== undefined && (obj.creator = message.creator);
    return obj;
  },

  fromPartial(object: DeepPartial<PoolPosition>): PoolPosition {
    const message = { ...basePoolPosition } as PoolPosition;
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
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
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
