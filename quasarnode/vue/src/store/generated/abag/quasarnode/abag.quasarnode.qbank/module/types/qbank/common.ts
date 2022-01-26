/* eslint-disable */
import { Coin } from "../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qbank";

export enum LockupTypes {
  Invalid = 0,
  /** Days_7 - 7 Days */
  Days_7 = 1,
  /** Days_21 - 21 Days of lockup */
  Days_21 = 2,
  /** Months_1 - 1 Month of lockup */
  Months_1 = 3,
  /** Months_3 - 3 Months of lockup */
  Months_3 = 4,
  UNRECOGNIZED = -1,
}

export function lockupTypesFromJSON(object: any): LockupTypes {
  switch (object) {
    case 0:
    case "Invalid":
      return LockupTypes.Invalid;
    case 1:
    case "Days_7":
      return LockupTypes.Days_7;
    case 2:
    case "Days_21":
      return LockupTypes.Days_21;
    case 3:
    case "Months_1":
      return LockupTypes.Months_1;
    case 4:
    case "Months_3":
      return LockupTypes.Months_3;
    case -1:
    case "UNRECOGNIZED":
    default:
      return LockupTypes.UNRECOGNIZED;
  }
}

export function lockupTypesToJSON(object: LockupTypes): string {
  switch (object) {
    case LockupTypes.Invalid:
      return "Invalid";
    case LockupTypes.Days_7:
      return "Days_7";
    case LockupTypes.Days_21:
      return "Days_21";
    case LockupTypes.Months_1:
      return "Months_1";
    case LockupTypes.Months_3:
      return "Months_3";
    default:
      return "UNKNOWN";
  }
}

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
