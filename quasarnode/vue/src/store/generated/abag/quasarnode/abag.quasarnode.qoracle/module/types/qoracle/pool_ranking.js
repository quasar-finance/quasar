/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qoracle";
const basePoolRanking = {
    poolIdsSortedByAPY: "",
    poolIdsSortedByTVL: "",
    lastUpdatedTime: 0,
    creator: "",
};
export const PoolRanking = {
    encode(message, writer = Writer.create()) {
        for (const v of message.poolIdsSortedByAPY) {
            writer.uint32(10).string(v);
        }
        for (const v of message.poolIdsSortedByTVL) {
            writer.uint32(18).string(v);
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(24).uint64(message.lastUpdatedTime);
        }
        if (message.creator !== "") {
            writer.uint32(34).string(message.creator);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...basePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolIdsSortedByAPY.push(reader.string());
                    break;
                case 2:
                    message.poolIdsSortedByTVL.push(reader.string());
                    break;
                case 3:
                    message.lastUpdatedTime = longToNumber(reader.uint64());
                    break;
                case 4:
                    message.creator = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...basePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        if (object.poolIdsSortedByAPY !== undefined &&
            object.poolIdsSortedByAPY !== null) {
            for (const e of object.poolIdsSortedByAPY) {
                message.poolIdsSortedByAPY.push(String(e));
            }
        }
        if (object.poolIdsSortedByTVL !== undefined &&
            object.poolIdsSortedByTVL !== null) {
            for (const e of object.poolIdsSortedByTVL) {
                message.poolIdsSortedByTVL.push(String(e));
            }
        }
        if (object.lastUpdatedTime !== undefined &&
            object.lastUpdatedTime !== null) {
            message.lastUpdatedTime = Number(object.lastUpdatedTime);
        }
        else {
            message.lastUpdatedTime = 0;
        }
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        if (message.poolIdsSortedByAPY) {
            obj.poolIdsSortedByAPY = message.poolIdsSortedByAPY.map((e) => e);
        }
        else {
            obj.poolIdsSortedByAPY = [];
        }
        if (message.poolIdsSortedByTVL) {
            obj.poolIdsSortedByTVL = message.poolIdsSortedByTVL.map((e) => e);
        }
        else {
            obj.poolIdsSortedByTVL = [];
        }
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        message.creator !== undefined && (obj.creator = message.creator);
        return obj;
    },
    fromPartial(object) {
        const message = { ...basePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        if (object.poolIdsSortedByAPY !== undefined &&
            object.poolIdsSortedByAPY !== null) {
            for (const e of object.poolIdsSortedByAPY) {
                message.poolIdsSortedByAPY.push(e);
            }
        }
        if (object.poolIdsSortedByTVL !== undefined &&
            object.poolIdsSortedByTVL !== null) {
            for (const e of object.poolIdsSortedByTVL) {
                message.poolIdsSortedByTVL.push(e);
            }
        }
        if (object.lastUpdatedTime !== undefined &&
            object.lastUpdatedTime !== null) {
            message.lastUpdatedTime = object.lastUpdatedTime;
        }
        else {
            message.lastUpdatedTime = 0;
        }
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
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
