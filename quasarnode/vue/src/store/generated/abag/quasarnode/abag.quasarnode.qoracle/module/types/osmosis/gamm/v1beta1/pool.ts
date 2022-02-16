/* eslint-disable */
import { Coin } from "../../../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.osmosis.gamm";

/** package osmosis.gamm.v1beta1; */

/**
 * option go_package = "github.com/advanced-blockchain/intergamm/x/gamm/types";
 * github.com/abag/quasarnode/x
 */
export interface PoolAsset {
  /**
   * Coins we are talking about,
   * the denomination must be unique amongst all PoolAssets for this pool.
   */
  token: Coin | undefined;
  /** Weight that is not normalized. This weight must be less than 2^50 */
  weight: string;
}

const basePoolAsset: object = { weight: "" };

export const PoolAsset = {
  encode(message: PoolAsset, writer: Writer = Writer.create()): Writer {
    if (message.token !== undefined) {
      Coin.encode(message.token, writer.uint32(10).fork()).ldelim();
    }
    if (message.weight !== "") {
      writer.uint32(18).string(message.weight);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): PoolAsset {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...basePoolAsset } as PoolAsset;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.token = Coin.decode(reader, reader.uint32());
          break;
        case 2:
          message.weight = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): PoolAsset {
    const message = { ...basePoolAsset } as PoolAsset;
    if (object.token !== undefined && object.token !== null) {
      message.token = Coin.fromJSON(object.token);
    } else {
      message.token = undefined;
    }
    if (object.weight !== undefined && object.weight !== null) {
      message.weight = String(object.weight);
    } else {
      message.weight = "";
    }
    return message;
  },

  toJSON(message: PoolAsset): unknown {
    const obj: any = {};
    message.token !== undefined &&
      (obj.token = message.token ? Coin.toJSON(message.token) : undefined);
    message.weight !== undefined && (obj.weight = message.weight);
    return obj;
  },

  fromPartial(object: DeepPartial<PoolAsset>): PoolAsset {
    const message = { ...basePoolAsset } as PoolAsset;
    if (object.token !== undefined && object.token !== null) {
      message.token = Coin.fromPartial(object.token);
    } else {
      message.token = undefined;
    }
    if (object.weight !== undefined && object.weight !== null) {
      message.weight = object.weight;
    } else {
      message.weight = "";
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
