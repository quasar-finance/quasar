/* eslint-disable */
import { Coin } from "../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qbank";

export interface QCoins {
  coins: Coin[];
}

export interface QDenoms {
  denoms: string[];
}

const baseQCoins: object = {};

export const QCoins = {
  encode(message: QCoins, writer: Writer = Writer.create()): Writer {
    for (const v of message.coins) {
      Coin.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QCoins {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseQCoins } as QCoins;
    message.coins = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.coins.push(Coin.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QCoins {
    const message = { ...baseQCoins } as QCoins;
    message.coins = [];
    if (object.coins !== undefined && object.coins !== null) {
      for (const e of object.coins) {
        message.coins.push(Coin.fromJSON(e));
      }
    }
    return message;
  },

  toJSON(message: QCoins): unknown {
    const obj: any = {};
    if (message.coins) {
      obj.coins = message.coins.map((e) => (e ? Coin.toJSON(e) : undefined));
    } else {
      obj.coins = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<QCoins>): QCoins {
    const message = { ...baseQCoins } as QCoins;
    message.coins = [];
    if (object.coins !== undefined && object.coins !== null) {
      for (const e of object.coins) {
        message.coins.push(Coin.fromPartial(e));
      }
    }
    return message;
  },
};

const baseQDenoms: object = { denoms: "" };

export const QDenoms = {
  encode(message: QDenoms, writer: Writer = Writer.create()): Writer {
    for (const v of message.denoms) {
      writer.uint32(10).string(v!);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QDenoms {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseQDenoms } as QDenoms;
    message.denoms = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.denoms.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QDenoms {
    const message = { ...baseQDenoms } as QDenoms;
    message.denoms = [];
    if (object.denoms !== undefined && object.denoms !== null) {
      for (const e of object.denoms) {
        message.denoms.push(String(e));
      }
    }
    return message;
  },

  toJSON(message: QDenoms): unknown {
    const obj: any = {};
    if (message.denoms) {
      obj.denoms = message.denoms.map((e) => e);
    } else {
      obj.denoms = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<QDenoms>): QDenoms {
    const message = { ...baseQDenoms } as QDenoms;
    message.denoms = [];
    if (object.denoms !== undefined && object.denoms !== null) {
      for (const e of object.denoms) {
        message.denoms.push(e);
      }
    }
    return message;
  },
};

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
