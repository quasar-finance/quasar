/* eslint-disable */
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qoracle";

export interface PoolMetrics {
  aPY: string;
  tVL: string;
}

const basePoolMetrics: object = { aPY: "", tVL: "" };

export const PoolMetrics = {
  encode(message: PoolMetrics, writer: Writer = Writer.create()): Writer {
    if (message.aPY !== "") {
      writer.uint32(10).string(message.aPY);
    }
    if (message.tVL !== "") {
      writer.uint32(18).string(message.tVL);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): PoolMetrics {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...basePoolMetrics } as PoolMetrics;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.aPY = reader.string();
          break;
        case 2:
          message.tVL = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PoolMetrics {
    const message = { ...basePoolMetrics } as PoolMetrics;
    if (object.aPY !== undefined && object.aPY !== null) {
      message.aPY = String(object.aPY);
    } else {
      message.aPY = "";
    }
    if (object.tVL !== undefined && object.tVL !== null) {
      message.tVL = String(object.tVL);
    } else {
      message.tVL = "";
    }
    return message;
  },

  toJSON(message: PoolMetrics): unknown {
    const obj: any = {};
    message.aPY !== undefined && (obj.aPY = message.aPY);
    message.tVL !== undefined && (obj.tVL = message.tVL);
    return obj;
  },

  fromPartial(object: DeepPartial<PoolMetrics>): PoolMetrics {
    const message = { ...basePoolMetrics } as PoolMetrics;
    if (object.aPY !== undefined && object.aPY !== null) {
      message.aPY = object.aPY;
    } else {
      message.aPY = "";
    }
    if (object.tVL !== undefined && object.tVL !== null) {
      message.tVL = object.tVL;
    } else {
      message.tVL = "";
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
