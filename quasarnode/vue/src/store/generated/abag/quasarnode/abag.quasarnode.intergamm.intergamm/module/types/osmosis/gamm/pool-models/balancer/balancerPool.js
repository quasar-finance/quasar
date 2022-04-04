/* eslint-disable */
import { Timestamp } from "../../../../google/protobuf/timestamp";
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { Duration } from "../../../../google/protobuf/duration";
import { PoolAsset } from "../../../../osmosis/gamm/v1beta1/pool";
import { Coin } from "../../../../cosmos/base/v1beta1/coin";
export const protobufPackage = "osmosis.gamm.poolmodels";
const baseSmoothWeightChangeParams = {};
export const SmoothWeightChangeParams = {
    encode(message, writer = Writer.create()) {
        if (message.startTime !== undefined) {
            Timestamp.encode(toTimestamp(message.startTime), writer.uint32(10).fork()).ldelim();
        }
        if (message.duration !== undefined) {
            Duration.encode(message.duration, writer.uint32(18).fork()).ldelim();
        }
        for (const v of message.initialPoolWeights) {
            PoolAsset.encode(v, writer.uint32(26).fork()).ldelim();
        }
        for (const v of message.targetPoolWeights) {
            PoolAsset.encode(v, writer.uint32(34).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseSmoothWeightChangeParams,
        };
        message.initialPoolWeights = [];
        message.targetPoolWeights = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.startTime = fromTimestamp(Timestamp.decode(reader, reader.uint32()));
                    break;
                case 2:
                    message.duration = Duration.decode(reader, reader.uint32());
                    break;
                case 3:
                    message.initialPoolWeights.push(PoolAsset.decode(reader, reader.uint32()));
                    break;
                case 4:
                    message.targetPoolWeights.push(PoolAsset.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = {
            ...baseSmoothWeightChangeParams,
        };
        message.initialPoolWeights = [];
        message.targetPoolWeights = [];
        if (object.startTime !== undefined && object.startTime !== null) {
            message.startTime = fromJsonTimestamp(object.startTime);
        }
        else {
            message.startTime = undefined;
        }
        if (object.duration !== undefined && object.duration !== null) {
            message.duration = Duration.fromJSON(object.duration);
        }
        else {
            message.duration = undefined;
        }
        if (object.initialPoolWeights !== undefined &&
            object.initialPoolWeights !== null) {
            for (const e of object.initialPoolWeights) {
                message.initialPoolWeights.push(PoolAsset.fromJSON(e));
            }
        }
        if (object.targetPoolWeights !== undefined &&
            object.targetPoolWeights !== null) {
            for (const e of object.targetPoolWeights) {
                message.targetPoolWeights.push(PoolAsset.fromJSON(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.startTime !== undefined &&
            (obj.startTime =
                message.startTime !== undefined
                    ? message.startTime.toISOString()
                    : null);
        message.duration !== undefined &&
            (obj.duration = message.duration
                ? Duration.toJSON(message.duration)
                : undefined);
        if (message.initialPoolWeights) {
            obj.initialPoolWeights = message.initialPoolWeights.map((e) => e ? PoolAsset.toJSON(e) : undefined);
        }
        else {
            obj.initialPoolWeights = [];
        }
        if (message.targetPoolWeights) {
            obj.targetPoolWeights = message.targetPoolWeights.map((e) => e ? PoolAsset.toJSON(e) : undefined);
        }
        else {
            obj.targetPoolWeights = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseSmoothWeightChangeParams,
        };
        message.initialPoolWeights = [];
        message.targetPoolWeights = [];
        if (object.startTime !== undefined && object.startTime !== null) {
            message.startTime = object.startTime;
        }
        else {
            message.startTime = undefined;
        }
        if (object.duration !== undefined && object.duration !== null) {
            message.duration = Duration.fromPartial(object.duration);
        }
        else {
            message.duration = undefined;
        }
        if (object.initialPoolWeights !== undefined &&
            object.initialPoolWeights !== null) {
            for (const e of object.initialPoolWeights) {
                message.initialPoolWeights.push(PoolAsset.fromPartial(e));
            }
        }
        if (object.targetPoolWeights !== undefined &&
            object.targetPoolWeights !== null) {
            for (const e of object.targetPoolWeights) {
                message.targetPoolWeights.push(PoolAsset.fromPartial(e));
            }
        }
        return message;
    },
};
const baseBalancerPoolParams = { swapFee: "", exitFee: "" };
export const BalancerPoolParams = {
    encode(message, writer = Writer.create()) {
        if (message.swapFee !== "") {
            writer.uint32(10).string(message.swapFee);
        }
        if (message.exitFee !== "") {
            writer.uint32(18).string(message.exitFee);
        }
        if (message.smoothWeightChangeParams !== undefined) {
            SmoothWeightChangeParams.encode(message.smoothWeightChangeParams, writer.uint32(26).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseBalancerPoolParams };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.swapFee = reader.string();
                    break;
                case 2:
                    message.exitFee = reader.string();
                    break;
                case 3:
                    message.smoothWeightChangeParams = SmoothWeightChangeParams.decode(reader, reader.uint32());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseBalancerPoolParams };
        if (object.swapFee !== undefined && object.swapFee !== null) {
            message.swapFee = String(object.swapFee);
        }
        else {
            message.swapFee = "";
        }
        if (object.exitFee !== undefined && object.exitFee !== null) {
            message.exitFee = String(object.exitFee);
        }
        else {
            message.exitFee = "";
        }
        if (object.smoothWeightChangeParams !== undefined &&
            object.smoothWeightChangeParams !== null) {
            message.smoothWeightChangeParams = SmoothWeightChangeParams.fromJSON(object.smoothWeightChangeParams);
        }
        else {
            message.smoothWeightChangeParams = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.swapFee !== undefined && (obj.swapFee = message.swapFee);
        message.exitFee !== undefined && (obj.exitFee = message.exitFee);
        message.smoothWeightChangeParams !== undefined &&
            (obj.smoothWeightChangeParams = message.smoothWeightChangeParams
                ? SmoothWeightChangeParams.toJSON(message.smoothWeightChangeParams)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseBalancerPoolParams };
        if (object.swapFee !== undefined && object.swapFee !== null) {
            message.swapFee = object.swapFee;
        }
        else {
            message.swapFee = "";
        }
        if (object.exitFee !== undefined && object.exitFee !== null) {
            message.exitFee = object.exitFee;
        }
        else {
            message.exitFee = "";
        }
        if (object.smoothWeightChangeParams !== undefined &&
            object.smoothWeightChangeParams !== null) {
            message.smoothWeightChangeParams = SmoothWeightChangeParams.fromPartial(object.smoothWeightChangeParams);
        }
        else {
            message.smoothWeightChangeParams = undefined;
        }
        return message;
    },
};
const baseBalancerPool = {
    address: "",
    id: 0,
    futurePoolGovernor: "",
    totalWeight: "",
};
export const BalancerPool = {
    encode(message, writer = Writer.create()) {
        if (message.address !== "") {
            writer.uint32(10).string(message.address);
        }
        if (message.id !== 0) {
            writer.uint32(16).uint64(message.id);
        }
        if (message.poolParams !== undefined) {
            BalancerPoolParams.encode(message.poolParams, writer.uint32(26).fork()).ldelim();
        }
        if (message.futurePoolGovernor !== "") {
            writer.uint32(34).string(message.futurePoolGovernor);
        }
        if (message.totalShares !== undefined) {
            Coin.encode(message.totalShares, writer.uint32(42).fork()).ldelim();
        }
        for (const v of message.poolAssets) {
            PoolAsset.encode(v, writer.uint32(50).fork()).ldelim();
        }
        if (message.totalWeight !== "") {
            writer.uint32(58).string(message.totalWeight);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseBalancerPool };
        message.poolAssets = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.address = reader.string();
                    break;
                case 2:
                    message.id = longToNumber(reader.uint64());
                    break;
                case 3:
                    message.poolParams = BalancerPoolParams.decode(reader, reader.uint32());
                    break;
                case 4:
                    message.futurePoolGovernor = reader.string();
                    break;
                case 5:
                    message.totalShares = Coin.decode(reader, reader.uint32());
                    break;
                case 6:
                    message.poolAssets.push(PoolAsset.decode(reader, reader.uint32()));
                    break;
                case 7:
                    message.totalWeight = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseBalancerPool };
        message.poolAssets = [];
        if (object.address !== undefined && object.address !== null) {
            message.address = String(object.address);
        }
        else {
            message.address = "";
        }
        if (object.id !== undefined && object.id !== null) {
            message.id = Number(object.id);
        }
        else {
            message.id = 0;
        }
        if (object.poolParams !== undefined && object.poolParams !== null) {
            message.poolParams = BalancerPoolParams.fromJSON(object.poolParams);
        }
        else {
            message.poolParams = undefined;
        }
        if (object.futurePoolGovernor !== undefined &&
            object.futurePoolGovernor !== null) {
            message.futurePoolGovernor = String(object.futurePoolGovernor);
        }
        else {
            message.futurePoolGovernor = "";
        }
        if (object.totalShares !== undefined && object.totalShares !== null) {
            message.totalShares = Coin.fromJSON(object.totalShares);
        }
        else {
            message.totalShares = undefined;
        }
        if (object.poolAssets !== undefined && object.poolAssets !== null) {
            for (const e of object.poolAssets) {
                message.poolAssets.push(PoolAsset.fromJSON(e));
            }
        }
        if (object.totalWeight !== undefined && object.totalWeight !== null) {
            message.totalWeight = String(object.totalWeight);
        }
        else {
            message.totalWeight = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.address !== undefined && (obj.address = message.address);
        message.id !== undefined && (obj.id = message.id);
        message.poolParams !== undefined &&
            (obj.poolParams = message.poolParams
                ? BalancerPoolParams.toJSON(message.poolParams)
                : undefined);
        message.futurePoolGovernor !== undefined &&
            (obj.futurePoolGovernor = message.futurePoolGovernor);
        message.totalShares !== undefined &&
            (obj.totalShares = message.totalShares
                ? Coin.toJSON(message.totalShares)
                : undefined);
        if (message.poolAssets) {
            obj.poolAssets = message.poolAssets.map((e) => e ? PoolAsset.toJSON(e) : undefined);
        }
        else {
            obj.poolAssets = [];
        }
        message.totalWeight !== undefined &&
            (obj.totalWeight = message.totalWeight);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseBalancerPool };
        message.poolAssets = [];
        if (object.address !== undefined && object.address !== null) {
            message.address = object.address;
        }
        else {
            message.address = "";
        }
        if (object.id !== undefined && object.id !== null) {
            message.id = object.id;
        }
        else {
            message.id = 0;
        }
        if (object.poolParams !== undefined && object.poolParams !== null) {
            message.poolParams = BalancerPoolParams.fromPartial(object.poolParams);
        }
        else {
            message.poolParams = undefined;
        }
        if (object.futurePoolGovernor !== undefined &&
            object.futurePoolGovernor !== null) {
            message.futurePoolGovernor = object.futurePoolGovernor;
        }
        else {
            message.futurePoolGovernor = "";
        }
        if (object.totalShares !== undefined && object.totalShares !== null) {
            message.totalShares = Coin.fromPartial(object.totalShares);
        }
        else {
            message.totalShares = undefined;
        }
        if (object.poolAssets !== undefined && object.poolAssets !== null) {
            for (const e of object.poolAssets) {
                message.poolAssets.push(PoolAsset.fromPartial(e));
            }
        }
        if (object.totalWeight !== undefined && object.totalWeight !== null) {
            message.totalWeight = object.totalWeight;
        }
        else {
            message.totalWeight = "";
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
function toTimestamp(date) {
    const seconds = date.getTime() / 1000;
    const nanos = (date.getTime() % 1000) * 1000000;
    return { seconds, nanos };
}
function fromTimestamp(t) {
    let millis = t.seconds * 1000;
    millis += t.nanos / 1000000;
    return new Date(millis);
}
function fromJsonTimestamp(o) {
    if (o instanceof Date) {
        return o;
    }
    else if (typeof o === "string") {
        return new Date(o);
    }
    else {
        return fromTimestamp(Timestamp.fromJSON(o));
    }
}
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
