/* eslint-disable */
import { Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qoracle";
const baseParams = { oracleAccounts: "" };
export const Params = {
    encode(message, writer = Writer.create()) {
        if (message.oracleAccounts !== "") {
            writer.uint32(10).string(message.oracleAccounts);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseParams };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.oracleAccounts = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseParams };
        if (object.oracleAccounts !== undefined && object.oracleAccounts !== null) {
            message.oracleAccounts = String(object.oracleAccounts);
        }
        else {
            message.oracleAccounts = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.oracleAccounts !== undefined &&
            (obj.oracleAccounts = message.oracleAccounts);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseParams };
        if (object.oracleAccounts !== undefined && object.oracleAccounts !== null) {
            message.oracleAccounts = object.oracleAccounts;
        }
        else {
            message.oracleAccounts = "";
        }
        return message;
    },
};
