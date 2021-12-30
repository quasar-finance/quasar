/* eslint-disable */
import * as Long from "long";
import { util, configure, Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qbank";
const baseDeposit = {
    id: 0,
    riskProfile: "",
    vaultID: "",
    depositorAccAddress: "",
    amount: "",
    denom: "",
};
export const Deposit = {
    encode(message, writer = Writer.create()) {
        if (message.id !== 0) {
            writer.uint32(8).uint64(message.id);
        }
        if (message.riskProfile !== "") {
            writer.uint32(18).string(message.riskProfile);
        }
        if (message.vaultID !== "") {
            writer.uint32(26).string(message.vaultID);
        }
        if (message.depositorAccAddress !== "") {
            writer.uint32(34).string(message.depositorAccAddress);
        }
        if (message.amount !== "") {
            writer.uint32(42).string(message.amount);
        }
        if (message.denom !== "") {
            writer.uint32(50).string(message.denom);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseDeposit };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.id = longToNumber(reader.uint64());
                    break;
                case 2:
                    message.riskProfile = reader.string();
                    break;
                case 3:
                    message.vaultID = reader.string();
                    break;
                case 4:
                    message.depositorAccAddress = reader.string();
                    break;
                case 5:
                    message.amount = reader.string();
                    break;
                case 6:
                    message.denom = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseDeposit };
        if (object.id !== undefined && object.id !== null) {
            message.id = Number(object.id);
        }
        else {
            message.id = 0;
        }
        if (object.riskProfile !== undefined && object.riskProfile !== null) {
            message.riskProfile = String(object.riskProfile);
        }
        else {
            message.riskProfile = "";
        }
        if (object.vaultID !== undefined && object.vaultID !== null) {
            message.vaultID = String(object.vaultID);
        }
        else {
            message.vaultID = "";
        }
        if (object.depositorAccAddress !== undefined &&
            object.depositorAccAddress !== null) {
            message.depositorAccAddress = String(object.depositorAccAddress);
        }
        else {
            message.depositorAccAddress = "";
        }
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = String(object.amount);
        }
        else {
            message.amount = "";
        }
        if (object.denom !== undefined && object.denom !== null) {
            message.denom = String(object.denom);
        }
        else {
            message.denom = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.id !== undefined && (obj.id = message.id);
        message.riskProfile !== undefined &&
            (obj.riskProfile = message.riskProfile);
        message.vaultID !== undefined && (obj.vaultID = message.vaultID);
        message.depositorAccAddress !== undefined &&
            (obj.depositorAccAddress = message.depositorAccAddress);
        message.amount !== undefined && (obj.amount = message.amount);
        message.denom !== undefined && (obj.denom = message.denom);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseDeposit };
        if (object.id !== undefined && object.id !== null) {
            message.id = object.id;
        }
        else {
            message.id = 0;
        }
        if (object.riskProfile !== undefined && object.riskProfile !== null) {
            message.riskProfile = object.riskProfile;
        }
        else {
            message.riskProfile = "";
        }
        if (object.vaultID !== undefined && object.vaultID !== null) {
            message.vaultID = object.vaultID;
        }
        else {
            message.vaultID = "";
        }
        if (object.depositorAccAddress !== undefined &&
            object.depositorAccAddress !== null) {
            message.depositorAccAddress = object.depositorAccAddress;
        }
        else {
            message.depositorAccAddress = "";
        }
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = object.amount;
        }
        else {
            message.amount = "";
        }
        if (object.denom !== undefined && object.denom !== null) {
            message.denom = object.denom;
        }
        else {
            message.denom = "";
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
