/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
import { Params } from "../qbank/params";
import { Deposit } from "../qbank/deposit";
import { Withdraw } from "../qbank/withdraw";
export const protobufPackage = "abag.quasarnode.qbank";
const baseGenesisState = { depositCount: 0, withdrawCount: 0 };
export const GenesisState = {
    encode(message, writer = Writer.create()) {
        if (message.params !== undefined) {
            Params.encode(message.params, writer.uint32(10).fork()).ldelim();
        }
        for (const v of message.depositList) {
            Deposit.encode(v, writer.uint32(18).fork()).ldelim();
        }
        if (message.depositCount !== 0) {
            writer.uint32(24).uint64(message.depositCount);
        }
        for (const v of message.withdrawList) {
            Withdraw.encode(v, writer.uint32(34).fork()).ldelim();
        }
        if (message.withdrawCount !== 0) {
            writer.uint32(40).uint64(message.withdrawCount);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseGenesisState };
        message.depositList = [];
        message.withdrawList = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.params = Params.decode(reader, reader.uint32());
                    break;
                case 2:
                    message.depositList.push(Deposit.decode(reader, reader.uint32()));
                    break;
                case 3:
                    message.depositCount = longToNumber(reader.uint64());
                    break;
                case 4:
                    message.withdrawList.push(Withdraw.decode(reader, reader.uint32()));
                    break;
                case 5:
                    message.withdrawCount = longToNumber(reader.uint64());
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
        message.depositList = [];
        message.withdrawList = [];
        if (object.params !== undefined && object.params !== null) {
            message.params = Params.fromJSON(object.params);
        }
        else {
            message.params = undefined;
        }
        if (object.depositList !== undefined && object.depositList !== null) {
            for (const e of object.depositList) {
                message.depositList.push(Deposit.fromJSON(e));
            }
        }
        if (object.depositCount !== undefined && object.depositCount !== null) {
            message.depositCount = Number(object.depositCount);
        }
        else {
            message.depositCount = 0;
        }
        if (object.withdrawList !== undefined && object.withdrawList !== null) {
            for (const e of object.withdrawList) {
                message.withdrawList.push(Withdraw.fromJSON(e));
            }
        }
        if (object.withdrawCount !== undefined && object.withdrawCount !== null) {
            message.withdrawCount = Number(object.withdrawCount);
        }
        else {
            message.withdrawCount = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.params !== undefined &&
            (obj.params = message.params ? Params.toJSON(message.params) : undefined);
        if (message.depositList) {
            obj.depositList = message.depositList.map((e) => e ? Deposit.toJSON(e) : undefined);
        }
        else {
            obj.depositList = [];
        }
        message.depositCount !== undefined &&
            (obj.depositCount = message.depositCount);
        if (message.withdrawList) {
            obj.withdrawList = message.withdrawList.map((e) => e ? Withdraw.toJSON(e) : undefined);
        }
        else {
            obj.withdrawList = [];
        }
        message.withdrawCount !== undefined &&
            (obj.withdrawCount = message.withdrawCount);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseGenesisState };
        message.depositList = [];
        message.withdrawList = [];
        if (object.params !== undefined && object.params !== null) {
            message.params = Params.fromPartial(object.params);
        }
        else {
            message.params = undefined;
        }
        if (object.depositList !== undefined && object.depositList !== null) {
            for (const e of object.depositList) {
                message.depositList.push(Deposit.fromPartial(e));
            }
        }
        if (object.depositCount !== undefined && object.depositCount !== null) {
            message.depositCount = object.depositCount;
        }
        else {
            message.depositCount = 0;
        }
        if (object.withdrawList !== undefined && object.withdrawList !== null) {
            for (const e of object.withdrawList) {
                message.withdrawList.push(Withdraw.fromPartial(e));
            }
        }
        if (object.withdrawCount !== undefined && object.withdrawCount !== null) {
            message.withdrawCount = object.withdrawCount;
        }
        else {
            message.withdrawCount = 0;
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
