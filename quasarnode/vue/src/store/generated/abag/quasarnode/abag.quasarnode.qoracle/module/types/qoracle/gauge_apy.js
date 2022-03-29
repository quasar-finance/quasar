/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qoracle";
const baseGaugeAPY = { gaugeId: 0, duration: "", aPY: "" };
export const GaugeAPY = {
    encode(message, writer = Writer.create()) {
        if (message.gaugeId !== 0) {
            writer.uint32(8).uint64(message.gaugeId);
        }
        if (message.duration !== "") {
            writer.uint32(18).string(message.duration);
        }
        if (message.aPY !== "") {
            writer.uint32(26).string(message.aPY);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseGaugeAPY };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.gaugeId = longToNumber(reader.uint64());
                    break;
                case 2:
                    message.duration = reader.string();
                    break;
                case 3:
                    message.aPY = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseGaugeAPY };
        if (object.gaugeId !== undefined && object.gaugeId !== null) {
            message.gaugeId = Number(object.gaugeId);
        }
        else {
            message.gaugeId = 0;
        }
        if (object.duration !== undefined && object.duration !== null) {
            message.duration = String(object.duration);
        }
        else {
            message.duration = "";
        }
        if (object.aPY !== undefined && object.aPY !== null) {
            message.aPY = String(object.aPY);
        }
        else {
            message.aPY = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.gaugeId !== undefined && (obj.gaugeId = message.gaugeId);
        message.duration !== undefined && (obj.duration = message.duration);
        message.aPY !== undefined && (obj.aPY = message.aPY);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseGaugeAPY };
        if (object.gaugeId !== undefined && object.gaugeId !== null) {
            message.gaugeId = object.gaugeId;
        }
        else {
            message.gaugeId = 0;
        }
        if (object.duration !== undefined && object.duration !== null) {
            message.duration = object.duration;
        }
        else {
            message.duration = "";
        }
        if (object.aPY !== undefined && object.aPY !== null) {
            message.aPY = object.aPY;
        }
        else {
            message.aPY = "";
        }
        return message;
    },
};
var globalThis = (() => {
    if (typeof globalThis !== "undefined")
        return globalThis;
    if (typeof self !== "undefined")
        return self;
    if (typeof window !== "undefined")
        return window;
    if (typeof global !== "undefined")
        return global;
    throw "Unable to locate global object";
})();
function longToNumber(long) {
    if (long.gt(Number.MAX_SAFE_INTEGER)) {
        throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
    }
    return long.toNumber();
}
if (util.Long !== Long) {
    util.Long = Long;
    configure();
}
