/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { BalancerPool } from "../osmosis/gamm/pool-models/balancer/balancerPool";

export const protobufPackage = "abag.quasarnode.qoracle";

export interface PoolInfo {
  poolId: string;
  info: BalancerPool | undefined;
  lastUpdatedTime: number;
  creator: string;
}

const basePoolInfo: object = { poolId: "", lastUpdatedTime: 0, creator: "" };

export const PoolInfo = {
  encode(message: PoolInfo, writer: Writer = Writer.create()): Writer {
    if (message.poolId !== "") {
      writer.uint32(10).string(message.poolId);
    }
    if (message.info !== undefined) {
      BalancerPool.encode(message.info, writer.uint32(18).fork()).ldelim();
    }
    if (message.lastUpdatedTime !== 0) {
      writer.uint32(24).uint64(message.lastUpdatedTime);
    }
    if (message.creator !== "") {
      writer.uint32(34).string(message.creator);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): PoolInfo {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...basePoolInfo } as PoolInfo;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolId = reader.string();
          break;
        case 2:
          message.info = BalancerPool.decode(reader, reader.uint32());
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

  fromJSON(object: any): PoolInfo {
    const message = { ...basePoolInfo } as PoolInfo;
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
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    return message;
  },

  toJSON(message: PoolInfo): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.info !== undefined &&
      (obj.info = message.info ? BalancerPool.toJSON(message.info) : undefined);
    message.lastUpdatedTime !== undefined &&
      (obj.lastUpdatedTime = message.lastUpdatedTime);
    message.creator !== undefined && (obj.creator = message.creator);
    return obj;
  },

  fromPartial(object: DeepPartial<PoolInfo>): PoolInfo {
    const message = { ...basePoolInfo } as PoolInfo;
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
