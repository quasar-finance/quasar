/* eslint-disable */
import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";
import { PoolRanking } from "../qoracle/pool_ranking";
import { PoolSpotPrice } from "../qoracle/pool_spot_price";
import { PoolInfo } from "../qoracle/pool_info";
import { Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qoracle";
const baseGenesisState = {};
export const GenesisState = {
    encode(message, writer = Writer.create()) {
        if (message.params !== undefined) {
            Params.encode(message.params, writer.uint32(10).fork()).ldelim();
        }
        for (const v of message.poolPositionList) {
            PoolPosition.encode(v, writer.uint32(18).fork()).ldelim();
        }
        if (message.poolRanking !== undefined) {
            PoolRanking.encode(message.poolRanking, writer.uint32(26).fork()).ldelim();
        }
        for (const v of message.poolSpotPriceList) {
            PoolSpotPrice.encode(v, writer.uint32(34).fork()).ldelim();
        }
        for (const v of message.poolInfoList) {
            PoolInfo.encode(v, writer.uint32(42).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseGenesisState };
        message.poolPositionList = [];
        message.poolSpotPriceList = [];
        message.poolInfoList = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.params = Params.decode(reader, reader.uint32());
                    break;
                case 2:
                    message.poolPositionList.push(PoolPosition.decode(reader, reader.uint32()));
                    break;
                case 3:
                    message.poolRanking = PoolRanking.decode(reader, reader.uint32());
                    break;
                case 4:
                    message.poolSpotPriceList.push(PoolSpotPrice.decode(reader, reader.uint32()));
                    break;
                case 5:
                    message.poolInfoList.push(PoolInfo.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseGenesisState };
        message.poolPositionList = [];
        message.poolSpotPriceList = [];
        message.poolInfoList = [];
        if (object.params !== undefined && object.params !== null) {
            message.params = Params.fromJSON(object.params);
        }
        else {
            message.params = undefined;
        }
        if (object.poolPositionList !== undefined &&
            object.poolPositionList !== null) {
            for (const e of object.poolPositionList) {
                message.poolPositionList.push(PoolPosition.fromJSON(e));
            }
        }
        if (object.poolRanking !== undefined && object.poolRanking !== null) {
            message.poolRanking = PoolRanking.fromJSON(object.poolRanking);
        }
        else {
            message.poolRanking = undefined;
        }
        if (object.poolSpotPriceList !== undefined &&
            object.poolSpotPriceList !== null) {
            for (const e of object.poolSpotPriceList) {
                message.poolSpotPriceList.push(PoolSpotPrice.fromJSON(e));
            }
        }
        if (object.poolInfoList !== undefined && object.poolInfoList !== null) {
            for (const e of object.poolInfoList) {
                message.poolInfoList.push(PoolInfo.fromJSON(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.params !== undefined &&
            (obj.params = message.params ? Params.toJSON(message.params) : undefined);
        if (message.poolPositionList) {
            obj.poolPositionList = message.poolPositionList.map((e) => e ? PoolPosition.toJSON(e) : undefined);
        }
        else {
            obj.poolPositionList = [];
        }
        message.poolRanking !== undefined &&
            (obj.poolRanking = message.poolRanking
                ? PoolRanking.toJSON(message.poolRanking)
                : undefined);
        if (message.poolSpotPriceList) {
            obj.poolSpotPriceList = message.poolSpotPriceList.map((e) => e ? PoolSpotPrice.toJSON(e) : undefined);
        }
        else {
            obj.poolSpotPriceList = [];
        }
        if (message.poolInfoList) {
            obj.poolInfoList = message.poolInfoList.map((e) => e ? PoolInfo.toJSON(e) : undefined);
        }
        else {
            obj.poolInfoList = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseGenesisState };
        message.poolPositionList = [];
        message.poolSpotPriceList = [];
        message.poolInfoList = [];
        if (object.params !== undefined && object.params !== null) {
            message.params = Params.fromPartial(object.params);
        }
        else {
            message.params = undefined;
        }
        if (object.poolPositionList !== undefined &&
            object.poolPositionList !== null) {
            for (const e of object.poolPositionList) {
                message.poolPositionList.push(PoolPosition.fromPartial(e));
            }
        }
        if (object.poolRanking !== undefined && object.poolRanking !== null) {
            message.poolRanking = PoolRanking.fromPartial(object.poolRanking);
        }
        else {
            message.poolRanking = undefined;
        }
        if (object.poolSpotPriceList !== undefined &&
            object.poolSpotPriceList !== null) {
            for (const e of object.poolSpotPriceList) {
                message.poolSpotPriceList.push(PoolSpotPrice.fromPartial(e));
            }
        }
        if (object.poolInfoList !== undefined && object.poolInfoList !== null) {
            for (const e of object.poolInfoList) {
                message.poolInfoList.push(PoolInfo.fromPartial(e));
            }
        }
        return message;
    },
};
