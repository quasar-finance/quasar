/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { PoolMetrics } from "../qoracle/pool_metrics";
import { BalancerPool } from "../osmosis/gamm/pool-models/balancer/balancerPool";
export const protobufPackage = "abag.quasarnode.qoracle";
const baseMsgCreatePoolPosition = {
    creator: "",
    poolId: "",
    lastUpdatedTime: 0,
};
export const MsgCreatePoolPosition = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        if (message.metrics !== undefined) {
            PoolMetrics.encode(message.metrics, writer.uint32(26).fork()).ldelim();
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(32).uint64(message.lastUpdatedTime);
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
                    message.poolId = reader.string();
                    break;
                case 3:
                    message.metrics = PoolMetrics.decode(reader, reader.uint32());
                    break;
                case 4:
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
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        if (object.metrics !== undefined && object.metrics !== null) {
            message.metrics = PoolMetrics.fromJSON(object.metrics);
        }
        else {
            message.metrics = undefined;
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.metrics !== undefined &&
            (obj.metrics = message.metrics
                ? PoolMetrics.toJSON(message.metrics)
                : undefined);
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
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        if (object.metrics !== undefined && object.metrics !== null) {
            message.metrics = PoolMetrics.fromPartial(object.metrics);
        }
        else {
            message.metrics = undefined;
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
    poolId: "",
    lastUpdatedTime: 0,
};
export const MsgUpdatePoolPosition = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        if (message.metrics !== undefined) {
            PoolMetrics.encode(message.metrics, writer.uint32(26).fork()).ldelim();
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(32).uint64(message.lastUpdatedTime);
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
                    message.poolId = reader.string();
                    break;
                case 3:
                    message.metrics = PoolMetrics.decode(reader, reader.uint32());
                    break;
                case 4:
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
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        if (object.metrics !== undefined && object.metrics !== null) {
            message.metrics = PoolMetrics.fromJSON(object.metrics);
        }
        else {
            message.metrics = undefined;
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.metrics !== undefined &&
            (obj.metrics = message.metrics
                ? PoolMetrics.toJSON(message.metrics)
                : undefined);
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
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        if (object.metrics !== undefined && object.metrics !== null) {
            message.metrics = PoolMetrics.fromPartial(object.metrics);
        }
        else {
            message.metrics = undefined;
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
const baseMsgDeletePoolPosition = { creator: "", poolId: "" };
export const MsgDeletePoolPosition = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
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
                    message.poolId = reader.string();
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
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolId !== undefined && (obj.poolId = message.poolId);
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
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
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
const baseMsgCreatePoolRanking = {
    creator: "",
    poolIdsSortedByAPY: "",
    poolIdsSortedByTVL: "",
    lastUpdatedTime: 0,
};
export const MsgCreatePoolRanking = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        for (const v of message.poolIdsSortedByAPY) {
            writer.uint32(26).string(v);
        }
        for (const v of message.poolIdsSortedByTVL) {
            writer.uint32(34).string(v);
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(40).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgCreatePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 3:
                    message.poolIdsSortedByAPY.push(reader.string());
                    break;
                case 4:
                    message.poolIdsSortedByTVL.push(reader.string());
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
        const message = { ...baseMsgCreatePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
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
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgCreatePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
};
const baseMsgCreatePoolRankingResponse = {};
export const MsgCreatePoolRankingResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgCreatePoolRankingResponse,
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
            ...baseMsgCreatePoolRankingResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgCreatePoolRankingResponse,
        };
        return message;
    },
};
const baseMsgUpdatePoolRanking = {
    creator: "",
    poolIdsSortedByAPY: "",
    poolIdsSortedByTVL: "",
    lastUpdatedTime: 0,
};
export const MsgUpdatePoolRanking = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        for (const v of message.poolIdsSortedByAPY) {
            writer.uint32(26).string(v);
        }
        for (const v of message.poolIdsSortedByTVL) {
            writer.uint32(34).string(v);
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(40).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgUpdatePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 3:
                    message.poolIdsSortedByAPY.push(reader.string());
                    break;
                case 4:
                    message.poolIdsSortedByTVL.push(reader.string());
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
        const message = { ...baseMsgUpdatePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
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
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgUpdatePoolRanking };
        message.poolIdsSortedByAPY = [];
        message.poolIdsSortedByTVL = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
};
const baseMsgUpdatePoolRankingResponse = {};
export const MsgUpdatePoolRankingResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgUpdatePoolRankingResponse,
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
            ...baseMsgUpdatePoolRankingResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgUpdatePoolRankingResponse,
        };
        return message;
    },
};
const baseMsgDeletePoolRanking = { creator: "" };
export const MsgDeletePoolRanking = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgDeletePoolRanking };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
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
        const message = { ...baseMsgDeletePoolRanking };
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
        message.creator !== undefined && (obj.creator = message.creator);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgDeletePoolRanking };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        return message;
    },
};
const baseMsgDeletePoolRankingResponse = {};
export const MsgDeletePoolRankingResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgDeletePoolRankingResponse,
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
            ...baseMsgDeletePoolRankingResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgDeletePoolRankingResponse,
        };
        return message;
    },
};
const baseMsgCreatePoolSpotPrice = {
    creator: "",
    poolId: "",
    denomIn: "",
    denomOut: "",
    price: "",
    lastUpdatedTime: 0,
};
export const MsgCreatePoolSpotPrice = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        if (message.denomIn !== "") {
            writer.uint32(26).string(message.denomIn);
        }
        if (message.denomOut !== "") {
            writer.uint32(34).string(message.denomOut);
        }
        if (message.price !== "") {
            writer.uint32(42).string(message.price);
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(48).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgCreatePoolSpotPrice };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolId = reader.string();
                    break;
                case 3:
                    message.denomIn = reader.string();
                    break;
                case 4:
                    message.denomOut = reader.string();
                    break;
                case 5:
                    message.price = reader.string();
                    break;
                case 6:
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
        const message = { ...baseMsgCreatePoolSpotPrice };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        if (object.denomIn !== undefined && object.denomIn !== null) {
            message.denomIn = String(object.denomIn);
        }
        else {
            message.denomIn = "";
        }
        if (object.denomOut !== undefined && object.denomOut !== null) {
            message.denomOut = String(object.denomOut);
        }
        else {
            message.denomOut = "";
        }
        if (object.price !== undefined && object.price !== null) {
            message.price = String(object.price);
        }
        else {
            message.price = "";
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.denomIn !== undefined && (obj.denomIn = message.denomIn);
        message.denomOut !== undefined && (obj.denomOut = message.denomOut);
        message.price !== undefined && (obj.price = message.price);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgCreatePoolSpotPrice };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        if (object.denomIn !== undefined && object.denomIn !== null) {
            message.denomIn = object.denomIn;
        }
        else {
            message.denomIn = "";
        }
        if (object.denomOut !== undefined && object.denomOut !== null) {
            message.denomOut = object.denomOut;
        }
        else {
            message.denomOut = "";
        }
        if (object.price !== undefined && object.price !== null) {
            message.price = object.price;
        }
        else {
            message.price = "";
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
const baseMsgCreatePoolSpotPriceResponse = {};
export const MsgCreatePoolSpotPriceResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgCreatePoolSpotPriceResponse,
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
            ...baseMsgCreatePoolSpotPriceResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgCreatePoolSpotPriceResponse,
        };
        return message;
    },
};
const baseMsgUpdatePoolSpotPrice = {
    creator: "",
    poolId: "",
    denomIn: "",
    denomOut: "",
    price: "",
    lastUpdatedTime: 0,
};
export const MsgUpdatePoolSpotPrice = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        if (message.denomIn !== "") {
            writer.uint32(26).string(message.denomIn);
        }
        if (message.denomOut !== "") {
            writer.uint32(34).string(message.denomOut);
        }
        if (message.price !== "") {
            writer.uint32(42).string(message.price);
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(48).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgUpdatePoolSpotPrice };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolId = reader.string();
                    break;
                case 3:
                    message.denomIn = reader.string();
                    break;
                case 4:
                    message.denomOut = reader.string();
                    break;
                case 5:
                    message.price = reader.string();
                    break;
                case 6:
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
        const message = { ...baseMsgUpdatePoolSpotPrice };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        if (object.denomIn !== undefined && object.denomIn !== null) {
            message.denomIn = String(object.denomIn);
        }
        else {
            message.denomIn = "";
        }
        if (object.denomOut !== undefined && object.denomOut !== null) {
            message.denomOut = String(object.denomOut);
        }
        else {
            message.denomOut = "";
        }
        if (object.price !== undefined && object.price !== null) {
            message.price = String(object.price);
        }
        else {
            message.price = "";
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.denomIn !== undefined && (obj.denomIn = message.denomIn);
        message.denomOut !== undefined && (obj.denomOut = message.denomOut);
        message.price !== undefined && (obj.price = message.price);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgUpdatePoolSpotPrice };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        if (object.denomIn !== undefined && object.denomIn !== null) {
            message.denomIn = object.denomIn;
        }
        else {
            message.denomIn = "";
        }
        if (object.denomOut !== undefined && object.denomOut !== null) {
            message.denomOut = object.denomOut;
        }
        else {
            message.denomOut = "";
        }
        if (object.price !== undefined && object.price !== null) {
            message.price = object.price;
        }
        else {
            message.price = "";
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
const baseMsgUpdatePoolSpotPriceResponse = {};
export const MsgUpdatePoolSpotPriceResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgUpdatePoolSpotPriceResponse,
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
            ...baseMsgUpdatePoolSpotPriceResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgUpdatePoolSpotPriceResponse,
        };
        return message;
    },
};
const baseMsgDeletePoolSpotPrice = {
    creator: "",
    poolId: "",
    denomIn: "",
    denomOut: "",
};
export const MsgDeletePoolSpotPrice = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        if (message.denomIn !== "") {
            writer.uint32(26).string(message.denomIn);
        }
        if (message.denomOut !== "") {
            writer.uint32(34).string(message.denomOut);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgDeletePoolSpotPrice };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolId = reader.string();
                    break;
                case 3:
                    message.denomIn = reader.string();
                    break;
                case 4:
                    message.denomOut = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgDeletePoolSpotPrice };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        if (object.denomIn !== undefined && object.denomIn !== null) {
            message.denomIn = String(object.denomIn);
        }
        else {
            message.denomIn = "";
        }
        if (object.denomOut !== undefined && object.denomOut !== null) {
            message.denomOut = String(object.denomOut);
        }
        else {
            message.denomOut = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.denomIn !== undefined && (obj.denomIn = message.denomIn);
        message.denomOut !== undefined && (obj.denomOut = message.denomOut);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgDeletePoolSpotPrice };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        if (object.denomIn !== undefined && object.denomIn !== null) {
            message.denomIn = object.denomIn;
        }
        else {
            message.denomIn = "";
        }
        if (object.denomOut !== undefined && object.denomOut !== null) {
            message.denomOut = object.denomOut;
        }
        else {
            message.denomOut = "";
        }
        return message;
    },
};
const baseMsgDeletePoolSpotPriceResponse = {};
export const MsgDeletePoolSpotPriceResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgDeletePoolSpotPriceResponse,
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
            ...baseMsgDeletePoolSpotPriceResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgDeletePoolSpotPriceResponse,
        };
        return message;
    },
};
const baseMsgCreatePoolInfo = {
    creator: "",
    poolId: "",
    lastUpdatedTime: 0,
};
export const MsgCreatePoolInfo = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        if (message.info !== undefined) {
            BalancerPool.encode(message.info, writer.uint32(26).fork()).ldelim();
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(32).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgCreatePoolInfo };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolId = reader.string();
                    break;
                case 3:
                    message.info = BalancerPool.decode(reader, reader.uint32());
                    break;
                case 4:
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
        const message = { ...baseMsgCreatePoolInfo };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.info !== undefined &&
            (obj.info = message.info ? BalancerPool.toJSON(message.info) : undefined);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgCreatePoolInfo };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
};
const baseMsgCreatePoolInfoResponse = {};
export const MsgCreatePoolInfoResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgCreatePoolInfoResponse,
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
            ...baseMsgCreatePoolInfoResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgCreatePoolInfoResponse,
        };
        return message;
    },
};
const baseMsgUpdatePoolInfo = {
    creator: "",
    poolId: "",
    lastUpdatedTime: 0,
};
export const MsgUpdatePoolInfo = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        if (message.info !== undefined) {
            BalancerPool.encode(message.info, writer.uint32(26).fork()).ldelim();
        }
        if (message.lastUpdatedTime !== 0) {
            writer.uint32(32).uint64(message.lastUpdatedTime);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgUpdatePoolInfo };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolId = reader.string();
                    break;
                case 3:
                    message.info = BalancerPool.decode(reader, reader.uint32());
                    break;
                case 4:
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
        const message = { ...baseMsgUpdatePoolInfo };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.info !== undefined &&
            (obj.info = message.info ? BalancerPool.toJSON(message.info) : undefined);
        message.lastUpdatedTime !== undefined &&
            (obj.lastUpdatedTime = message.lastUpdatedTime);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgUpdatePoolInfo };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
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
        return message;
    },
};
const baseMsgUpdatePoolInfoResponse = {};
export const MsgUpdatePoolInfoResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgUpdatePoolInfoResponse,
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
            ...baseMsgUpdatePoolInfoResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgUpdatePoolInfoResponse,
        };
        return message;
    },
};
const baseMsgDeletePoolInfo = { creator: "", poolId: "" };
export const MsgDeletePoolInfo = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.poolId !== "") {
            writer.uint32(18).string(message.poolId);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgDeletePoolInfo };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.poolId = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgDeletePoolInfo };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = String(object.poolId);
        }
        else {
            message.poolId = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.poolId !== undefined && (obj.poolId = message.poolId);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgDeletePoolInfo };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        return message;
    },
};
const baseMsgDeletePoolInfoResponse = {};
export const MsgDeletePoolInfoResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgDeletePoolInfoResponse,
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
            ...baseMsgDeletePoolInfoResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgDeletePoolInfoResponse,
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
    CreatePoolRanking(request) {
        const data = MsgCreatePoolRanking.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "CreatePoolRanking", data);
        return promise.then((data) => MsgCreatePoolRankingResponse.decode(new Reader(data)));
    }
    UpdatePoolRanking(request) {
        const data = MsgUpdatePoolRanking.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "UpdatePoolRanking", data);
        return promise.then((data) => MsgUpdatePoolRankingResponse.decode(new Reader(data)));
    }
    DeletePoolRanking(request) {
        const data = MsgDeletePoolRanking.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "DeletePoolRanking", data);
        return promise.then((data) => MsgDeletePoolRankingResponse.decode(new Reader(data)));
    }
    CreatePoolSpotPrice(request) {
        const data = MsgCreatePoolSpotPrice.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "CreatePoolSpotPrice", data);
        return promise.then((data) => MsgCreatePoolSpotPriceResponse.decode(new Reader(data)));
    }
    UpdatePoolSpotPrice(request) {
        const data = MsgUpdatePoolSpotPrice.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "UpdatePoolSpotPrice", data);
        return promise.then((data) => MsgUpdatePoolSpotPriceResponse.decode(new Reader(data)));
    }
    DeletePoolSpotPrice(request) {
        const data = MsgDeletePoolSpotPrice.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "DeletePoolSpotPrice", data);
        return promise.then((data) => MsgDeletePoolSpotPriceResponse.decode(new Reader(data)));
    }
    CreatePoolInfo(request) {
        const data = MsgCreatePoolInfo.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "CreatePoolInfo", data);
        return promise.then((data) => MsgCreatePoolInfoResponse.decode(new Reader(data)));
    }
    UpdatePoolInfo(request) {
        const data = MsgUpdatePoolInfo.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "UpdatePoolInfo", data);
        return promise.then((data) => MsgUpdatePoolInfoResponse.decode(new Reader(data)));
    }
    DeletePoolInfo(request) {
        const data = MsgDeletePoolInfo.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Msg", "DeletePoolInfo", data);
        return promise.then((data) => MsgDeletePoolInfoResponse.decode(new Reader(data)));
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
