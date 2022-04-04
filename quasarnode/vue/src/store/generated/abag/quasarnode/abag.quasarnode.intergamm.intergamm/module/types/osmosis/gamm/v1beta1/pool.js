/* eslint-disable */
import { Coin } from "../../../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "osmosis.gamm.v1beta1";
const basePoolAsset = { weight: "" };
export const PoolAsset = {
    encode(message, writer = Writer.create()) {
        if (message.token !== undefined) {
            Coin.encode(message.token, writer.uint32(10).fork()).ldelim();
        }
        if (message.weight !== "") {
            writer.uint32(18).string(message.weight);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...basePoolAsset };
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
    fromJSON(object) {
        const message = { ...basePoolAsset };
        if (object.token !== undefined && object.token !== null) {
            message.token = Coin.fromJSON(object.token);
        }
        else {
            message.token = undefined;
        }
        if (object.weight !== undefined && object.weight !== null) {
            message.weight = String(object.weight);
        }
        else {
            message.weight = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.token !== undefined &&
            (obj.token = message.token ? Coin.toJSON(message.token) : undefined);
        message.weight !== undefined && (obj.weight = message.weight);
        return obj;
    },
    fromPartial(object) {
        const message = { ...basePoolAsset };
        if (object.token !== undefined && object.token !== null) {
            message.token = Coin.fromPartial(object.token);
        }
        else {
            message.token = undefined;
        }
        if (object.weight !== undefined && object.weight !== null) {
            message.weight = object.weight;
        }
        else {
            message.weight = "";
        }
        return message;
    },
};
