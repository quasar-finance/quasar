/* eslint-disable */
import { Reader, Writer } from "protobufjs/minimal";
import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";
import { PageRequest, PageResponse, } from "../cosmos/base/query/v1beta1/pagination";
import { PoolRanking } from "../qoracle/pool_ranking";
import { PoolSpotPrice } from "../qoracle/pool_spot_price";
import { PoolInfo } from "../qoracle/pool_info";
export const protobufPackage = "abag.quasarnode.qoracle";
const baseQueryParamsRequest = {};
export const QueryParamsRequest = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseQueryParamsRequest };
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
        const message = { ...baseQueryParamsRequest };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = { ...baseQueryParamsRequest };
        return message;
    },
};
const baseQueryParamsResponse = {};
export const QueryParamsResponse = {
    encode(message, writer = Writer.create()) {
        if (message.params !== undefined) {
            Params.encode(message.params, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseQueryParamsResponse };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.params = Params.decode(reader, reader.uint32());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseQueryParamsResponse };
        if (object.params !== undefined && object.params !== null) {
            message.params = Params.fromJSON(object.params);
        }
        else {
            message.params = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.params !== undefined &&
            (obj.params = message.params ? Params.toJSON(message.params) : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseQueryParamsResponse };
        if (object.params !== undefined && object.params !== null) {
            message.params = Params.fromPartial(object.params);
        }
        else {
            message.params = undefined;
        }
        return message;
    },
};
const baseQueryGetPoolPositionRequest = { poolId: "" };
export const QueryGetPoolPositionRequest = {
    encode(message, writer = Writer.create()) {
        if (message.poolId !== "") {
            writer.uint32(10).string(message.poolId);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolPositionRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
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
        const message = {
            ...baseQueryGetPoolPositionRequest,
        };
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolPositionRequest,
        };
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        return message;
    },
};
const baseQueryGetPoolPositionResponse = {};
export const QueryGetPoolPositionResponse = {
    encode(message, writer = Writer.create()) {
        if (message.poolPosition !== undefined) {
            PoolPosition.encode(message.poolPosition, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolPositionResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolPosition = PoolPosition.decode(reader, reader.uint32());
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
            ...baseQueryGetPoolPositionResponse,
        };
        if (object.poolPosition !== undefined && object.poolPosition !== null) {
            message.poolPosition = PoolPosition.fromJSON(object.poolPosition);
        }
        else {
            message.poolPosition = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.poolPosition !== undefined &&
            (obj.poolPosition = message.poolPosition
                ? PoolPosition.toJSON(message.poolPosition)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolPositionResponse,
        };
        if (object.poolPosition !== undefined && object.poolPosition !== null) {
            message.poolPosition = PoolPosition.fromPartial(object.poolPosition);
        }
        else {
            message.poolPosition = undefined;
        }
        return message;
    },
};
const baseQueryAllPoolPositionRequest = {};
export const QueryAllPoolPositionRequest = {
    encode(message, writer = Writer.create()) {
        if (message.pagination !== undefined) {
            PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryAllPoolPositionRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.pagination = PageRequest.decode(reader, reader.uint32());
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
            ...baseQueryAllPoolPositionRequest,
        };
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageRequest.fromJSON(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageRequest.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllPoolPositionRequest,
        };
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageRequest.fromPartial(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
};
const baseQueryAllPoolPositionResponse = {};
export const QueryAllPoolPositionResponse = {
    encode(message, writer = Writer.create()) {
        for (const v of message.poolPosition) {
            PoolPosition.encode(v, writer.uint32(10).fork()).ldelim();
        }
        if (message.pagination !== undefined) {
            PageResponse.encode(message.pagination, writer.uint32(18).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryAllPoolPositionResponse,
        };
        message.poolPosition = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolPosition.push(PoolPosition.decode(reader, reader.uint32()));
                    break;
                case 2:
                    message.pagination = PageResponse.decode(reader, reader.uint32());
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
            ...baseQueryAllPoolPositionResponse,
        };
        message.poolPosition = [];
        if (object.poolPosition !== undefined && object.poolPosition !== null) {
            for (const e of object.poolPosition) {
                message.poolPosition.push(PoolPosition.fromJSON(e));
            }
        }
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageResponse.fromJSON(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        if (message.poolPosition) {
            obj.poolPosition = message.poolPosition.map((e) => e ? PoolPosition.toJSON(e) : undefined);
        }
        else {
            obj.poolPosition = [];
        }
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageResponse.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllPoolPositionResponse,
        };
        message.poolPosition = [];
        if (object.poolPosition !== undefined && object.poolPosition !== null) {
            for (const e of object.poolPosition) {
                message.poolPosition.push(PoolPosition.fromPartial(e));
            }
        }
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageResponse.fromPartial(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
};
const baseQueryGetPoolRankingRequest = {};
export const QueryGetPoolRankingRequest = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolRankingRequest,
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
            ...baseQueryGetPoolRankingRequest,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseQueryGetPoolRankingRequest,
        };
        return message;
    },
};
const baseQueryGetPoolRankingResponse = {};
export const QueryGetPoolRankingResponse = {
    encode(message, writer = Writer.create()) {
        if (message.PoolRanking !== undefined) {
            PoolRanking.encode(message.PoolRanking, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolRankingResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.PoolRanking = PoolRanking.decode(reader, reader.uint32());
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
            ...baseQueryGetPoolRankingResponse,
        };
        if (object.PoolRanking !== undefined && object.PoolRanking !== null) {
            message.PoolRanking = PoolRanking.fromJSON(object.PoolRanking);
        }
        else {
            message.PoolRanking = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.PoolRanking !== undefined &&
            (obj.PoolRanking = message.PoolRanking
                ? PoolRanking.toJSON(message.PoolRanking)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolRankingResponse,
        };
        if (object.PoolRanking !== undefined && object.PoolRanking !== null) {
            message.PoolRanking = PoolRanking.fromPartial(object.PoolRanking);
        }
        else {
            message.PoolRanking = undefined;
        }
        return message;
    },
};
const baseQueryGetPoolSpotPriceRequest = {
    poolId: "",
    denomIn: "",
    denomOut: "",
};
export const QueryGetPoolSpotPriceRequest = {
    encode(message, writer = Writer.create()) {
        if (message.poolId !== "") {
            writer.uint32(10).string(message.poolId);
        }
        if (message.denomIn !== "") {
            writer.uint32(18).string(message.denomIn);
        }
        if (message.denomOut !== "") {
            writer.uint32(26).string(message.denomOut);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolSpotPriceRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolId = reader.string();
                    break;
                case 2:
                    message.denomIn = reader.string();
                    break;
                case 3:
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
        const message = {
            ...baseQueryGetPoolSpotPriceRequest,
        };
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.denomIn !== undefined && (obj.denomIn = message.denomIn);
        message.denomOut !== undefined && (obj.denomOut = message.denomOut);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolSpotPriceRequest,
        };
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
const baseQueryGetPoolSpotPriceResponse = {};
export const QueryGetPoolSpotPriceResponse = {
    encode(message, writer = Writer.create()) {
        if (message.poolSpotPrice !== undefined) {
            PoolSpotPrice.encode(message.poolSpotPrice, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolSpotPriceResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolSpotPrice = PoolSpotPrice.decode(reader, reader.uint32());
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
            ...baseQueryGetPoolSpotPriceResponse,
        };
        if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
            message.poolSpotPrice = PoolSpotPrice.fromJSON(object.poolSpotPrice);
        }
        else {
            message.poolSpotPrice = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.poolSpotPrice !== undefined &&
            (obj.poolSpotPrice = message.poolSpotPrice
                ? PoolSpotPrice.toJSON(message.poolSpotPrice)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolSpotPriceResponse,
        };
        if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
            message.poolSpotPrice = PoolSpotPrice.fromPartial(object.poolSpotPrice);
        }
        else {
            message.poolSpotPrice = undefined;
        }
        return message;
    },
};
const baseQueryAllPoolSpotPriceRequest = {};
export const QueryAllPoolSpotPriceRequest = {
    encode(message, writer = Writer.create()) {
        if (message.pagination !== undefined) {
            PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryAllPoolSpotPriceRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.pagination = PageRequest.decode(reader, reader.uint32());
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
            ...baseQueryAllPoolSpotPriceRequest,
        };
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageRequest.fromJSON(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageRequest.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllPoolSpotPriceRequest,
        };
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageRequest.fromPartial(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
};
const baseQueryAllPoolSpotPriceResponse = {};
export const QueryAllPoolSpotPriceResponse = {
    encode(message, writer = Writer.create()) {
        for (const v of message.poolSpotPrice) {
            PoolSpotPrice.encode(v, writer.uint32(10).fork()).ldelim();
        }
        if (message.pagination !== undefined) {
            PageResponse.encode(message.pagination, writer.uint32(18).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryAllPoolSpotPriceResponse,
        };
        message.poolSpotPrice = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolSpotPrice.push(PoolSpotPrice.decode(reader, reader.uint32()));
                    break;
                case 2:
                    message.pagination = PageResponse.decode(reader, reader.uint32());
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
            ...baseQueryAllPoolSpotPriceResponse,
        };
        message.poolSpotPrice = [];
        if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
            for (const e of object.poolSpotPrice) {
                message.poolSpotPrice.push(PoolSpotPrice.fromJSON(e));
            }
        }
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageResponse.fromJSON(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        if (message.poolSpotPrice) {
            obj.poolSpotPrice = message.poolSpotPrice.map((e) => e ? PoolSpotPrice.toJSON(e) : undefined);
        }
        else {
            obj.poolSpotPrice = [];
        }
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageResponse.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllPoolSpotPriceResponse,
        };
        message.poolSpotPrice = [];
        if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
            for (const e of object.poolSpotPrice) {
                message.poolSpotPrice.push(PoolSpotPrice.fromPartial(e));
            }
        }
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageResponse.fromPartial(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
};
const baseQueryGetPoolInfoRequest = { poolId: "" };
export const QueryGetPoolInfoRequest = {
    encode(message, writer = Writer.create()) {
        if (message.poolId !== "") {
            writer.uint32(10).string(message.poolId);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolInfoRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
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
        const message = {
            ...baseQueryGetPoolInfoRequest,
        };
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
        message.poolId !== undefined && (obj.poolId = message.poolId);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolInfoRequest,
        };
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = "";
        }
        return message;
    },
};
const baseQueryGetPoolInfoResponse = {};
export const QueryGetPoolInfoResponse = {
    encode(message, writer = Writer.create()) {
        if (message.poolInfo !== undefined) {
            PoolInfo.encode(message.poolInfo, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolInfoResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolInfo = PoolInfo.decode(reader, reader.uint32());
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
            ...baseQueryGetPoolInfoResponse,
        };
        if (object.poolInfo !== undefined && object.poolInfo !== null) {
            message.poolInfo = PoolInfo.fromJSON(object.poolInfo);
        }
        else {
            message.poolInfo = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.poolInfo !== undefined &&
            (obj.poolInfo = message.poolInfo
                ? PoolInfo.toJSON(message.poolInfo)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolInfoResponse,
        };
        if (object.poolInfo !== undefined && object.poolInfo !== null) {
            message.poolInfo = PoolInfo.fromPartial(object.poolInfo);
        }
        else {
            message.poolInfo = undefined;
        }
        return message;
    },
};
const baseQueryAllPoolInfoRequest = {};
export const QueryAllPoolInfoRequest = {
    encode(message, writer = Writer.create()) {
        if (message.pagination !== undefined) {
            PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryAllPoolInfoRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.pagination = PageRequest.decode(reader, reader.uint32());
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
            ...baseQueryAllPoolInfoRequest,
        };
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageRequest.fromJSON(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageRequest.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllPoolInfoRequest,
        };
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageRequest.fromPartial(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
};
const baseQueryAllPoolInfoResponse = {};
export const QueryAllPoolInfoResponse = {
    encode(message, writer = Writer.create()) {
        for (const v of message.poolInfo) {
            PoolInfo.encode(v, writer.uint32(10).fork()).ldelim();
        }
        if (message.pagination !== undefined) {
            PageResponse.encode(message.pagination, writer.uint32(18).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryAllPoolInfoResponse,
        };
        message.poolInfo = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.poolInfo.push(PoolInfo.decode(reader, reader.uint32()));
                    break;
                case 2:
                    message.pagination = PageResponse.decode(reader, reader.uint32());
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
            ...baseQueryAllPoolInfoResponse,
        };
        message.poolInfo = [];
        if (object.poolInfo !== undefined && object.poolInfo !== null) {
            for (const e of object.poolInfo) {
                message.poolInfo.push(PoolInfo.fromJSON(e));
            }
        }
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageResponse.fromJSON(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        if (message.poolInfo) {
            obj.poolInfo = message.poolInfo.map((e) => e ? PoolInfo.toJSON(e) : undefined);
        }
        else {
            obj.poolInfo = [];
        }
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageResponse.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllPoolInfoResponse,
        };
        message.poolInfo = [];
        if (object.poolInfo !== undefined && object.poolInfo !== null) {
            for (const e of object.poolInfo) {
                message.poolInfo.push(PoolInfo.fromPartial(e));
            }
        }
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageResponse.fromPartial(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
};
export class QueryClientImpl {
    constructor(rpc) {
        this.rpc = rpc;
    }
    Params(request) {
        const data = QueryParamsRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "Params", data);
        return promise.then((data) => QueryParamsResponse.decode(new Reader(data)));
    }
    PoolPosition(request) {
        const data = QueryGetPoolPositionRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolPosition", data);
        return promise.then((data) => QueryGetPoolPositionResponse.decode(new Reader(data)));
    }
    PoolPositionAll(request) {
        const data = QueryAllPoolPositionRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolPositionAll", data);
        return promise.then((data) => QueryAllPoolPositionResponse.decode(new Reader(data)));
    }
    PoolRanking(request) {
        const data = QueryGetPoolRankingRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolRanking", data);
        return promise.then((data) => QueryGetPoolRankingResponse.decode(new Reader(data)));
    }
    PoolSpotPrice(request) {
        const data = QueryGetPoolSpotPriceRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolSpotPrice", data);
        return promise.then((data) => QueryGetPoolSpotPriceResponse.decode(new Reader(data)));
    }
    PoolSpotPriceAll(request) {
        const data = QueryAllPoolSpotPriceRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolSpotPriceAll", data);
        return promise.then((data) => QueryAllPoolSpotPriceResponse.decode(new Reader(data)));
    }
    PoolInfo(request) {
        const data = QueryGetPoolInfoRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolInfo", data);
        return promise.then((data) => QueryGetPoolInfoResponse.decode(new Reader(data)));
    }
    PoolInfoAll(request) {
        const data = QueryAllPoolInfoRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolInfoAll", data);
        return promise.then((data) => QueryAllPoolInfoResponse.decode(new Reader(data)));
    }
}
