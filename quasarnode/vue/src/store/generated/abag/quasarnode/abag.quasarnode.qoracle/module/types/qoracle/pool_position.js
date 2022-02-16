/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qoracle";
const baseSortedPools = { ID: 0 };
export const SortedPools = {
    encode(message, writer = Writer.create()) {
        writer.uint32(10).fork();
        for (const v of message.ID) {
            writer.uint64(v);
        }
        writer.ldelim();
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseSortedPools };
        message.ID = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    if ((tag & 7) === 2) {
                        const end2 = reader.uint32() + reader.pos;
                        while (reader.pos < end2) {
                            message.ID.push(longToNumber(reader.uint64()));
                        }
                    }
                    else {
                        message.ID.push(longToNumber(reader.uint64()));
                    }
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseSortedPools };
        message.ID = [];
        if (object.ID !== undefined && object.ID !== null) {
            for (const e of object.ID) {
                message.ID.push(Number(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        if (message.ID) {
            obj.ID = message.ID.map((e) => e);
        }
        else {
            obj.ID = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseSortedPools };
        message.ID = [];
        if (object.ID !== undefined && object.ID !== null) {
            for (const e of object.ID) {
                message.ID.push(e);
            }
        }
        return message;
    },
};
const basePoolPosition = {
    aPY: 0,
    tVL: 0,
    lastUpdatedTime: 0,
    creator: "",
};
export const PoolPosition = {
    encode(message, writer = Writer.create()) {
        if (message.aPY !== 0) {
            writer.uint32(8).uint64(message.aPY);
        }
        if (message.tVL !== 0) {
            writer.uint32(16).uint64(message.tVL);
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
        const message = { ...basePoolPosition };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.aPY = longToNumber(reader.uint64());
                    break;
                case 2:
                    message.tVL = longToNumber(reader.uint64());
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
        const message = { ...basePoolPosition };
        if (object.aPY !== undefined && object.aPY !== null) {
            message.aPY = Number(object.aPY);
        }
        else {
            message.aPY = 0;
        }
        if (object.tVL !== undefined && object.tVL !== null) {
            message.tVL = Number(object.tVL);
        }
        else {
            message.tVL = 0;
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
        message.aPY !== undefined && (obj.aPY = message.aPY);
        message.tVL !== undefined && (obj.tVL = message.tVL);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        message.creator !== undefined && (obj.creator = message.creator);
        return obj;
    },
    fromPartial(object) {
        const message = { ...basePoolPosition };
        if (object.aPY !== undefined && object.aPY !== null) {
            message.aPY = object.aPY;
        }
        else {
            message.aPY = 0;
        }
        if (object.tVL !== undefined && object.tVL !== null) {
            message.tVL = object.tVL;
        }
        else {
            message.tVL = 0;
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
