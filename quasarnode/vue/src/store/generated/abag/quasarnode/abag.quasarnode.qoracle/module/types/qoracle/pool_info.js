/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { BalancerPool } from "../osmosis/gamm/pool-models/balancer/balancerPool";
export const protobufPackage = "abag.quasarnode.qoracle";
const basePoolInfo = { poolId: "", lastUpdatedTime: 0, creator: "" };
export const PoolInfo = {
    encode(message, writer = Writer.create()) {
        if (message.poolId !== "") {
            writer.uint32(10).string(message.poolId);
        }
        if (message.info !== undefined) {
            BalancerPool.encode(message.info, writer.uint32(18).fork()).ldelim();
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
        const message = { ...basePoolInfo };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolId = reader.string();
                    break;
                case 2:
                    message.info = BalancerPool.decode(reader, reader.uint32());
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
        const message = { ...basePoolInfo };
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        if (object.info !== undefined && object.info !== null) {
            message.info = BalancerPool.fromJSON(object.info);
        }
        else {
            message.info = undefined;
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.info !== undefined &&
            (obj.info = message.info ? BalancerPool.toJSON(message.info) : undefined);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        message.creator !== undefined && (obj.creator = message.creator);
        return obj;
    },
    fromPartial(object) {
        const message = { ...basePoolInfo };
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        if (object.info !== undefined && object.info !== null) {
            message.info = BalancerPool.fromPartial(object.info);
        }
        else {
            message.info = undefined;
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
