/* eslint-disable */
import { Reader, Writer } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qbank";
const baseMsgRequestDeposit = {
    creator: "",
    riskProfile: "",
    vaultID: "",
    amount: "",
    denom: "",
};
export const MsgRequestDeposit = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.riskProfile !== "") {
            writer.uint32(18).string(message.riskProfile);
        }
        if (message.vaultID !== "") {
            writer.uint32(26).string(message.vaultID);
        }
        if (message.amount !== "") {
            writer.uint32(34).string(message.amount);
        }
        if (message.denom !== "") {
            writer.uint32(42).string(message.denom);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgRequestDeposit };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.riskProfile = reader.string();
                    break;
                case 3:
                    message.vaultID = reader.string();
                    break;
                case 4:
                    message.amount = reader.string();
                    break;
                case 5:
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
        const message = { ...baseMsgRequestDeposit };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
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
        message.creator !== undefined && (obj.creator = message.creator);
        message.riskProfile !== undefined &&
            (obj.riskProfile = message.riskProfile);
        message.vaultID !== undefined && (obj.vaultID = message.vaultID);
        message.amount !== undefined && (obj.amount = message.amount);
        message.denom !== undefined && (obj.denom = message.denom);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgRequestDeposit };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
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
const baseMsgRequestDepositResponse = {};
export const MsgRequestDepositResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgRequestDepositResponse,
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
            ...baseMsgRequestDepositResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgRequestDepositResponse,
        };
        return message;
    },
};
export class MsgClientImpl {
    constructor(rpc) {
        this.rpc = rpc;
    }
    RequestDeposit(request) {
        const data = MsgRequestDeposit.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Msg", "RequestDeposit", data);
        return promise.then((data) => MsgRequestDepositResponse.decode(new Reader(data)));
    }
}
