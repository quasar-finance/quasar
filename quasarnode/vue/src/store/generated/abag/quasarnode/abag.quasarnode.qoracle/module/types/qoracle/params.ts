/* eslint-disable */
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qoracle";

/** Params defines the parameters for the module. */
export interface Params {
  oracleAccounts: string;
}

const baseParams: object = { oracleAccounts: "" };

export const Params = {
  encode(message: Params, writer: Writer = Writer.create()): Writer {
    if (message.oracleAccounts !== "") {
      writer.uint32(10).string(message.oracleAccounts);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Params {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseParams } as Params;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.oracleAccounts = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Params {
    const message = { ...baseParams } as Params;
    if (object.oracleAccounts !== undefined && object.oracleAccounts !== null) {
      message.oracleAccounts = String(object.oracleAccounts);
    } else {
      message.oracleAccounts = "";
    }
    return message;
  },

  toJSON(message: Params): unknown {
    const obj: any = {};
    message.oracleAccounts !== undefined &&
      (obj.oracleAccounts = message.oracleAccounts);
    return obj;
  },

  fromPartial(object: DeepPartial<Params>): Params {
    const message = { ...baseParams } as Params;
    if (object.oracleAccounts !== undefined && object.oracleAccounts !== null) {
      message.oracleAccounts = object.oracleAccounts;
    } else {
      message.oracleAccounts = "";
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
