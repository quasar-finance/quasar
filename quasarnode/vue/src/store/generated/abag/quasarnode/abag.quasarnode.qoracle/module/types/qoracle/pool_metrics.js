/* eslint-disable */
import { Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qoracle";
const basePoolMetrics = { aPY: "", tVL: "" };
export const PoolMetrics = {
    encode(message, writer = Writer.create()) {
        if (message.aPY !== "") {
            writer.uint32(10).string(message.aPY);
        }
        if (message.tVL !== "") {
            writer.uint32(18).string(message.tVL);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...basePoolMetrics };
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
    fromJSON(object) {
        const message = { ...basePoolMetrics };
        if (object.aPY !== undefined && object.aPY !== null) {
            message.aPY = String(object.aPY);
        }
        else {
            message.aPY = "";
        }
        if (object.tVL !== undefined && object.tVL !== null) {
            message.tVL = String(object.tVL);
        }
        else {
            message.tVL = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.aPY !== undefined && (obj.aPY = message.aPY);
        message.tVL !== undefined && (obj.tVL = message.tVL);
        return obj;
    },
    fromPartial(object) {
        const message = { ...basePoolMetrics };
        if (object.aPY !== undefined && object.aPY !== null) {
            message.aPY = object.aPY;
        }
        else {
            message.aPY = "";
        }
        if (object.tVL !== undefined && object.tVL !== null) {
            message.tVL = object.tVL;
        }
        else {
            message.tVL = "";
        }
        return message;
    },
};
