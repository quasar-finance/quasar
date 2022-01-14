/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";

export const protobufPackage = "abag.quasarnode.qbank";

export interface FeeData {
  feeCollector: string;
  fromAddress: string;
  fee: Coin | undefined;
  feeType: number;
  blockHeight: number;
  memo: string;
}

const baseFeeData: object = {
  feeCollector: "",
  fromAddress: "",
  feeType: 0,
  blockHeight: 0,
  memo: "",
};

export const FeeData = {
  encode(message: FeeData, writer: Writer = Writer.create()): Writer {
    if (message.feeCollector !== "") {
      writer.uint32(10).string(message.feeCollector);
    }
    if (message.fromAddress !== "") {
      writer.uint32(18).string(message.fromAddress);
    }
    if (message.fee !== undefined) {
      Coin.encode(message.fee, writer.uint32(26).fork()).ldelim();
    }
    if (message.feeType !== 0) {
      writer.uint32(32).uint64(message.feeType);
    }
    if (message.blockHeight !== 0) {
      writer.uint32(40).uint64(message.blockHeight);
    }
    if (message.memo !== "") {
      writer.uint32(50).string(message.memo);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): FeeData {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseFeeData } as FeeData;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.feeCollector = reader.string();
          break;
        case 2:
          message.fromAddress = reader.string();
          break;
        case 3:
          message.fee = Coin.decode(reader, reader.uint32());
          break;
        case 4:
          message.feeType = longToNumber(reader.uint64() as Long);
          break;
        case 5:
          message.blockHeight = longToNumber(reader.uint64() as Long);
          break;
        case 6:
          message.memo = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FeeData {
    const message = { ...baseFeeData } as FeeData;
    if (object.feeCollector !== undefined && object.feeCollector !== null) {
      message.feeCollector = String(object.feeCollector);
    } else {
      message.feeCollector = "";
    }
    if (object.fromAddress !== undefined && object.fromAddress !== null) {
      message.fromAddress = String(object.fromAddress);
    } else {
      message.fromAddress = "";
    }
    if (object.fee !== undefined && object.fee !== null) {
      message.fee = Coin.fromJSON(object.fee);
    } else {
      message.fee = undefined;
    }
    if (object.feeType !== undefined && object.feeType !== null) {
      message.feeType = Number(object.feeType);
    } else {
      message.feeType = 0;
    }
    if (object.blockHeight !== undefined && object.blockHeight !== null) {
      message.blockHeight = Number(object.blockHeight);
    } else {
      message.blockHeight = 0;
    }
    if (object.memo !== undefined && object.memo !== null) {
      message.memo = String(object.memo);
    } else {
      message.memo = "";
    }
    return message;
  },

  toJSON(message: FeeData): unknown {
    const obj: any = {};
    message.feeCollector !== undefined &&
      (obj.feeCollector = message.feeCollector);
    message.fromAddress !== undefined &&
      (obj.fromAddress = message.fromAddress);
    message.fee !== undefined &&
      (obj.fee = message.fee ? Coin.toJSON(message.fee) : undefined);
    message.feeType !== undefined && (obj.feeType = message.feeType);
    message.blockHeight !== undefined &&
      (obj.blockHeight = message.blockHeight);
    message.memo !== undefined && (obj.memo = message.memo);
    return obj;
  },

  fromPartial(object: DeepPartial<FeeData>): FeeData {
    const message = { ...baseFeeData } as FeeData;
    if (object.feeCollector !== undefined && object.feeCollector !== null) {
      message.feeCollector = object.feeCollector;
    } else {
      message.feeCollector = "";
    }
    if (object.fromAddress !== undefined && object.fromAddress !== null) {
      message.fromAddress = object.fromAddress;
    } else {
      message.fromAddress = "";
    }
    if (object.fee !== undefined && object.fee !== null) {
      message.fee = Coin.fromPartial(object.fee);
    } else {
      message.fee = undefined;
    }
    if (object.feeType !== undefined && object.feeType !== null) {
      message.feeType = object.feeType;
    } else {
      message.feeType = 0;
    }
    if (object.blockHeight !== undefined && object.blockHeight !== null) {
      message.blockHeight = object.blockHeight;
    } else {
      message.blockHeight = 0;
    }
    if (object.memo !== undefined && object.memo !== null) {
      message.memo = object.memo;
    } else {
      message.memo = "";
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
