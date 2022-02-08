/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";

export const protobufPackage = "abag.quasarnode.qbank";

/** Withdraw defines the withdraw object to be stored in the KV store. */
export interface Withdraw {
  id: number;
  riskProfile: string;
  vaultID: string;
  depositorAccAddress: string;
  /**
   * string amount = 5;
   * string denom = 6;
   */
  coin: Coin | undefined;
}

const baseWithdraw: object = {
  id: 0,
  riskProfile: "",
  vaultID: "",
  depositorAccAddress: "",
};

export const Withdraw = {
  encode(message: Withdraw, writer: Writer = Writer.create()): Writer {
    if (message.id !== 0) {
      writer.uint32(8).uint64(message.id);
    }
    if (message.riskProfile !== "") {
      writer.uint32(18).string(message.riskProfile);
    }
    if (message.vaultID !== "") {
      writer.uint32(26).string(message.vaultID);
    }
    if (message.depositorAccAddress !== "") {
      writer.uint32(34).string(message.depositorAccAddress);
    }
    if (message.coin !== undefined) {
      Coin.encode(message.coin, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Withdraw {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseWithdraw } as Withdraw;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = longToNumber(reader.uint64() as Long);
          break;
        case 2:
          message.riskProfile = reader.string();
          break;
        case 3:
          message.vaultID = reader.string();
          break;
        case 4:
          message.depositorAccAddress = reader.string();
          break;
        case 5:
          message.coin = Coin.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Withdraw {
    const message = { ...baseWithdraw } as Withdraw;
    if (object.id !== undefined && object.id !== null) {
      message.id = Number(object.id);
    } else {
      message.id = 0;
    }
    if (object.riskProfile !== undefined && object.riskProfile !== null) {
      message.riskProfile = String(object.riskProfile);
    } else {
      message.riskProfile = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = String(object.vaultID);
    } else {
      message.vaultID = "";
    }
    if (
      object.depositorAccAddress !== undefined &&
      object.depositorAccAddress !== null
    ) {
      message.depositorAccAddress = String(object.depositorAccAddress);
    } else {
      message.depositorAccAddress = "";
    }
    if (object.coin !== undefined && object.coin !== null) {
      message.coin = Coin.fromJSON(object.coin);
    } else {
      message.coin = undefined;
    }
    return message;
  },

  toJSON(message: Withdraw): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.riskProfile !== undefined &&
      (obj.riskProfile = message.riskProfile);
    message.vaultID !== undefined && (obj.vaultID = message.vaultID);
    message.depositorAccAddress !== undefined &&
      (obj.depositorAccAddress = message.depositorAccAddress);
    message.coin !== undefined &&
      (obj.coin = message.coin ? Coin.toJSON(message.coin) : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<Withdraw>): Withdraw {
    const message = { ...baseWithdraw } as Withdraw;
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    } else {
      message.id = 0;
    }
    if (object.riskProfile !== undefined && object.riskProfile !== null) {
      message.riskProfile = object.riskProfile;
    } else {
      message.riskProfile = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = object.vaultID;
    } else {
      message.vaultID = "";
    }
    if (
      object.depositorAccAddress !== undefined &&
      object.depositorAccAddress !== null
    ) {
      message.depositorAccAddress = object.depositorAccAddress;
    } else {
      message.depositorAccAddress = "";
    }
    if (object.coin !== undefined && object.coin !== null) {
      message.coin = Coin.fromPartial(object.coin);
    } else {
      message.coin = undefined;
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
