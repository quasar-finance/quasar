/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { Params } from "../qbank/params";
import { Deposit } from "../qbank/deposit";
import { Withdraw } from "../qbank/withdraw";
import { FeeData } from "../qbank/fee_data";

export const protobufPackage = "abag.quasarnode.qbank";

/** GenesisState defines the qbank module's genesis state. */
export interface GenesisState {
  params: Params | undefined;
  depositList: Deposit[];
  depositCount: number;
  withdrawList: Withdraw[];
  withdrawCount: number;
  /** this line is used by starport scaffolding # genesis/proto/state */
  feeData: FeeData | undefined;
}

const baseGenesisState: object = { depositCount: 0, withdrawCount: 0 };

export const GenesisState = {
  encode(message: GenesisState, writer: Writer = Writer.create()): Writer {
    if (message.params !== undefined) {
      Params.encode(message.params, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.depositList) {
      Deposit.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.depositCount !== 0) {
      writer.uint32(24).uint64(message.depositCount);
    }
    for (const v of message.withdrawList) {
      Withdraw.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    if (message.withdrawCount !== 0) {
      writer.uint32(40).uint64(message.withdrawCount);
    }
    if (message.feeData !== undefined) {
      FeeData.encode(message.feeData, writer.uint32(50).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): GenesisState {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseGenesisState } as GenesisState;
    message.depositList = [];
    message.withdrawList = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.params = Params.decode(reader, reader.uint32());
          break;
        case 2:
          message.depositList.push(Deposit.decode(reader, reader.uint32()));
          break;
        case 3:
          message.depositCount = longToNumber(reader.uint64() as Long);
          break;
        case 4:
          message.withdrawList.push(Withdraw.decode(reader, reader.uint32()));
          break;
        case 5:
          message.withdrawCount = longToNumber(reader.uint64() as Long);
          break;
        case 6:
          message.feeData = FeeData.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GenesisState {
    const message = { ...baseGenesisState } as GenesisState;
    message.depositList = [];
    message.withdrawList = [];
    if (object.params !== undefined && object.params !== null) {
      message.params = Params.fromJSON(object.params);
    } else {
      message.params = undefined;
    }
    if (object.depositList !== undefined && object.depositList !== null) {
      for (const e of object.depositList) {
        message.depositList.push(Deposit.fromJSON(e));
      }
    }
    if (object.depositCount !== undefined && object.depositCount !== null) {
      message.depositCount = Number(object.depositCount);
    } else {
      message.depositCount = 0;
    }
    if (object.withdrawList !== undefined && object.withdrawList !== null) {
      for (const e of object.withdrawList) {
        message.withdrawList.push(Withdraw.fromJSON(e));
      }
    }
    if (object.withdrawCount !== undefined && object.withdrawCount !== null) {
      message.withdrawCount = Number(object.withdrawCount);
    } else {
      message.withdrawCount = 0;
    }
    if (object.feeData !== undefined && object.feeData !== null) {
      message.feeData = FeeData.fromJSON(object.feeData);
    } else {
      message.feeData = undefined;
    }
    return message;
  },

  toJSON(message: GenesisState): unknown {
    const obj: any = {};
    message.params !== undefined &&
      (obj.params = message.params ? Params.toJSON(message.params) : undefined);
    if (message.depositList) {
      obj.depositList = message.depositList.map((e) =>
        e ? Deposit.toJSON(e) : undefined
      );
    } else {
      obj.depositList = [];
    }
    message.depositCount !== undefined &&
      (obj.depositCount = message.depositCount);
    if (message.withdrawList) {
      obj.withdrawList = message.withdrawList.map((e) =>
        e ? Withdraw.toJSON(e) : undefined
      );
    } else {
      obj.withdrawList = [];
    }
    message.withdrawCount !== undefined &&
      (obj.withdrawCount = message.withdrawCount);
    message.feeData !== undefined &&
      (obj.feeData = message.feeData
        ? FeeData.toJSON(message.feeData)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<GenesisState>): GenesisState {
    const message = { ...baseGenesisState } as GenesisState;
    message.depositList = [];
    message.withdrawList = [];
    if (object.params !== undefined && object.params !== null) {
      message.params = Params.fromPartial(object.params);
    } else {
      message.params = undefined;
    }
    if (object.depositList !== undefined && object.depositList !== null) {
      for (const e of object.depositList) {
        message.depositList.push(Deposit.fromPartial(e));
      }
    }
    if (object.depositCount !== undefined && object.depositCount !== null) {
      message.depositCount = object.depositCount;
    } else {
      message.depositCount = 0;
    }
    if (object.withdrawList !== undefined && object.withdrawList !== null) {
      for (const e of object.withdrawList) {
        message.withdrawList.push(Withdraw.fromPartial(e));
      }
    }
    if (object.withdrawCount !== undefined && object.withdrawCount !== null) {
      message.withdrawCount = object.withdrawCount;
    } else {
      message.withdrawCount = 0;
    }
    if (object.feeData !== undefined && object.feeData !== null) {
      message.feeData = FeeData.fromPartial(object.feeData);
    } else {
      message.feeData = undefined;
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
