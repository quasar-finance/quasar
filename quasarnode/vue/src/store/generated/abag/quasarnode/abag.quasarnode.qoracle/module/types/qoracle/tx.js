/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
export const protobufPackage = "abag.quasarnode.qoracle";
const baseMsgCreatePoolPosition = {
    creator: "",
    poolID: 0,
    aPY: 0,
    tVL: 0,
    lastUpdatedTime: 0,
};
export const MsgCreatePoolPosition = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolID !== 0) {
            writer.uint32(16).uint64(message.poolID);
        }
        if (message.aPY !== 0) {
            writer.uint32(24).uint64(message.aPY);
        }
        if (message.tVL !== 0) {
            writer.uint32(32).uint64(message.tVL);
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(40).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgCreatePoolPosition };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolID = longToNumber(reader.uint64());
                    break;
                case 3:
                    message.aPY = longToNumber(reader.uint64());
                    break;
                case 4:
                    message.tVL = longToNumber(reader.uint64());
                    break;
                case 5:
                    message.lastUpdatedTime = longToNumber(reader.uint64());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgCreatePoolPosition };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.poolID !== undefined && object.poolID !== null) {
            message.poolID = Number(object.poolID);
        }
        else {
            message.poolID = 0;
        }
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
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolID !== undefined && (obj.poolID = message.poolID);
        message.aPY !== undefined && (obj.aPY = message.aPY);
        message.tVL !== undefined && (obj.tVL = message.tVL);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgCreatePoolPosition };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.poolID !== undefined && object.poolID !== null) {
            message.poolID = object.poolID;
        }
        else {
            message.poolID = 0;
        }
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
        return message;
    },
};
const baseMsgCreatePoolPositionResponse = {};
export const MsgCreatePoolPositionResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgCreatePoolPositionResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(_) {
        const message = {
            ...baseMsgCreatePoolPositionResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgCreatePoolPositionResponse,
        };
        return message;
    },
};
const baseMsgUpdatePoolPosition = {
    creator: "",
    poolID: 0,
    aPY: 0,
    tVL: 0,
    lastUpdatedTime: 0,
};
export const MsgUpdatePoolPosition = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolID !== 0) {
            writer.uint32(16).uint64(message.poolID);
        }
        if (message.aPY !== 0) {
            writer.uint32(24).uint64(message.aPY);
        }
        if (message.tVL !== 0) {
            writer.uint32(32).uint64(message.tVL);
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(40).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgUpdatePoolPosition };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolID = longToNumber(reader.uint64());
                    break;
                case 3:
                    message.aPY = longToNumber(reader.uint64());
                    break;
                case 4:
                    message.tVL = longToNumber(reader.uint64());
                    break;
                case 5:
                    message.lastUpdatedTime = longToNumber(reader.uint64());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgUpdatePoolPosition };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.poolID !== undefined && object.poolID !== null) {
            message.poolID = Number(object.poolID);
        }
        else {
            message.poolID = 0;
        }
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
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolID !== undefined && (obj.poolID = message.poolID);
        message.aPY !== undefined && (obj.aPY = message.aPY);
        message.tVL !== undefined && (obj.tVL = message.tVL);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgUpdatePoolPosition };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.poolID !== undefined && object.poolID !== null) {
            message.poolID = object.poolID;
        }
        else {
            message.poolID = 0;
        }
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
        return message;
    },
};
const baseMsgUpdatePoolPositionResponse = {};
export const MsgUpdatePoolPositionResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgUpdatePoolPositionResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(_) {
        const message = {
            ...baseMsgUpdatePoolPositionResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgUpdatePoolPositionResponse,
        };
        return message;
    },
};
const baseMsgDeletePoolPosition = { creator: "", poolID: 0 };
export const MsgDeletePoolPosition = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolID !== 0) {
            writer.uint32(16).uint64(message.poolID);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgDeletePoolPosition };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolID = longToNumber(reader.uint64());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgDeletePoolPosition };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.poolID !== undefined && object.poolID !== null) {
            message.poolID = Number(object.poolID);
        }
        else {
            message.poolID = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolID !== undefined && (obj.poolID = message.poolID);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgDeletePoolPosition };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.poolID !== undefined && object.poolID !== null) {
            message.poolID = object.poolID;
        }
        else {
            message.poolID = 0;
        }
        return message;
    },
};
const baseMsgDeletePoolPositionResponse = {};
export const MsgDeletePoolPositionResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgDeletePoolPositionResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(_) {
        const message = {
            ...baseMsgDeletePoolPositionResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgDeletePoolPositionResponse,
        };
        return message;
    },
};
export class MsgClientImpl {
    constructor(rpc) {
        this.rpc = rpc;
    }
    CreatePoolPosition(request) {
        const data = MsgCreatePoolPosition.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "CreatePoolPosition", data);
        return promise.then((data) => MsgCreatePoolPositionResponse.decode(new Reader(data)));
    }
    UpdatePoolPosition(request) {
        const data = MsgUpdatePoolPosition.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "UpdatePoolPosition", data);
        return promise.then((data) => MsgUpdatePoolPositionResponse.decode(new Reader(data)));
    }
    DeletePoolPosition(request) {
        const data = MsgDeletePoolPosition.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "DeletePoolPosition", data);
        return promise.then((data) => MsgDeletePoolPositionResponse.decode(new Reader(data)));
    }
}
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
