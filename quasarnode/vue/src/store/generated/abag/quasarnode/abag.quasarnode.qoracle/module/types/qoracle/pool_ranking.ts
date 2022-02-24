/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qoracle";

export interface PoolRanking {
  poolIdsSortedByAPY: string[];
  poolIdsSortedByTVL: string[];
  lastUpdatedTime: number;
  creator: string;
}

const basePoolRanking: object = {
  poolIdsSortedByAPY: "",
  poolIdsSortedByTVL: "",
  lastUpdatedTime: 0,
  creator: "",
};

export const PoolRanking = {
  encode(message: PoolRanking, writer: Writer = Writer.create()): Writer {
    for (const v of message.poolIdsSortedByAPY) {
      writer.uint32(10).string(v!);
    }
    for (const v of message.poolIdsSortedByTVL) {
      writer.uint32(18).string(v!);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(24).uint64(message.lastUpdatedTime);
    }
    if (message.creator !== "") {
      writer.uint32(34).string(message.creator);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): PoolRanking {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...basePoolRanking } as PoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolIdsSortedByAPY.push(reader.string());
          break;
        case 2:
          message.poolIdsSortedByTVL.push(reader.string());
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

  fromJSON(object: any): PoolRanking {
    const message = { ...basePoolRanking } as PoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
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
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    return message;
  },

  toJSON(message: PoolRanking): unknown {
    const obj: any = {};
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
    message.creator !== undefined && (obj.creator = message.creator);
    return obj;
  },

  fromPartial(object: DeepPartial<PoolRanking>): PoolRanking {
    const message = { ...basePoolRanking } as PoolRanking;
    message.poolIdsSortedByAPY = [];
    message.poolIdsSortedByTVL = [];
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
