/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qoracle";

export interface PoolSpotPrice {
  poolId: string;
  denomIn: string;
  denomOut: string;
  price: string;
  lastUpdatedTime: number;
  creator: string;
}

const basePoolSpotPrice: object = {
  poolId: "",
  denomIn: "",
  denomOut: "",
  price: "",
  lastUpdatedTime: 0,
  creator: "",
};

export const PoolSpotPrice = {
  encode(message: PoolSpotPrice, writer: Writer = Writer.create()): Writer {
    if (message.poolId !== "") {
      writer.uint32(10).string(message.poolId);
    }
    if (message.denomIn !== "") {
      writer.uint32(18).string(message.denomIn);
    }
    if (message.denomOut !== "") {
      writer.uint32(26).string(message.denomOut);
    }
    if (message.price !== "") {
      writer.uint32(34).string(message.price);
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(40).uint64(message.lastUpdatedTime);
    }
    if (message.creator !== "") {
      writer.uint32(50).string(message.creator);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): PoolSpotPrice {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...basePoolSpotPrice } as PoolSpotPrice;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolId = reader.string();
          break;
        case 2:
          message.denomIn = reader.string();
          break;
        case 3:
          message.denomOut = reader.string();
          break;
        case 4:
          message.price = reader.string();
          break;
        case 5:
          message.lastUpdatedTime = longToNumber(reader.uint64() as Long);
          break;
        case 6:
          message.creator = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PoolSpotPrice {
    const message = { ...basePoolSpotPrice } as PoolSpotPrice;
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
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    return message;
  },

  toJSON(message: PoolSpotPrice): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.denomIn !== undefined && (obj.denomIn = message.denomIn);
    message.denomOut !== undefined && (obj.denomOut = message.denomOut);
    message.price !== undefined && (obj.price = message.price);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    message.creator !== undefined && (obj.creator = message.creator);
    return obj;
  },

  fromPartial(object: DeepPartial<PoolSpotPrice>): PoolSpotPrice {
    const message = { ...basePoolSpotPrice } as PoolSpotPrice;
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
