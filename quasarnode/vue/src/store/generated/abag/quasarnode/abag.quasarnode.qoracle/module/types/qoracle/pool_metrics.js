/* eslint-disable */
import { GaugeAPY } from "../qoracle/gauge_apy";
import { Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qoracle";
const basePoolMetrics = { highestAPY: "", tVL: "" };
export const PoolMetrics = {
    encode(message, writer = Writer.create()) {
        if (message.highestAPY !== "") {
            writer.uint32(10).string(message.highestAPY);
        }
        if (message.tVL !== "") {
            writer.uint32(18).string(message.tVL);
        }
        for (const v of message.gaugeAPYs) {
            GaugeAPY.encode(v, writer.uint32(26).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...basePoolMetrics };
        message.gaugeAPYs = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.highestAPY = reader.string();
                    break;
                case 2:
                    message.tVL = reader.string();
                    break;
                case 3:
                    message.gaugeAPYs.push(GaugeAPY.decode(reader, reader.uint32()));
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
        message.gaugeAPYs = [];
        if (object.highestAPY !== undefined && object.highestAPY !== null) {
            message.highestAPY = String(object.highestAPY);
        }
        else {
            message.highestAPY = "";
        }
        if (object.tVL !== undefined && object.tVL !== null) {
            message.tVL = String(object.tVL);
        }
        else {
            message.tVL = "";
        }
        if (object.gaugeAPYs !== undefined && object.gaugeAPYs !== null) {
            for (const e of object.gaugeAPYs) {
                message.gaugeAPYs.push(GaugeAPY.fromJSON(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.highestAPY !== undefined && (obj.highestAPY = message.highestAPY);
        message.tVL !== undefined && (obj.tVL = message.tVL);
        if (message.gaugeAPYs) {
            obj.gaugeAPYs = message.gaugeAPYs.map((e) => e ? GaugeAPY.toJSON(e) : undefined);
        }
        else {
            obj.gaugeAPYs = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...basePoolMetrics };
        message.gaugeAPYs = [];
        if (object.highestAPY !== undefined && object.highestAPY !== null) {
            message.highestAPY = object.highestAPY;
        }
        else {
            message.highestAPY = "";
        }
        if (object.tVL !== undefined && object.tVL !== null) {
            message.tVL = object.tVL;
        }
        else {
            message.tVL = "";
        }
        if (object.gaugeAPYs !== undefined && object.gaugeAPYs !== null) {
            for (const e of object.gaugeAPYs) {
                message.gaugeAPYs.push(GaugeAPY.fromPartial(e));
            }
        }
        return message;
    },
};
