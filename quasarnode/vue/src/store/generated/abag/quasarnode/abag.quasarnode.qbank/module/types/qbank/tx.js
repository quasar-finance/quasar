/* eslint-disable */
import { lockupTypesFromJSON, lockupTypesToJSON, } from "../qbank/common";
import { Reader, Writer } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";
export const protobufPackage = "abag.quasarnode.qbank";
const baseMsgRequestDeposit = {
    creator: "",
    riskProfile: "",
    vaultID: "",
    lockupPeriod: 0,
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
        if (message.coin !== undefined) {
            Coin.encode(message.coin, writer.uint32(34).fork()).ldelim();
        }
        if (message.lockupPeriod !== 0) {
            writer.uint32(40).int32(message.lockupPeriod);
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
                    message.coin = Coin.decode(reader, reader.uint32());
                    break;
                case 5:
                    message.lockupPeriod = reader.int32();
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
        if (object.coin !== undefined && object.coin !== null) {
            message.coin = Coin.fromJSON(object.coin);
        }
        else {
            message.coin = undefined;
        }
        if (object.lockupPeriod !== undefined && object.lockupPeriod !== null) {
            message.lockupPeriod = lockupTypesFromJSON(object.lockupPeriod);
        }
        else {
            message.lockupPeriod = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.riskProfile !== undefined &&
            (obj.riskProfile = message.riskProfile);
        message.vaultID !== undefined && (obj.vaultID = message.vaultID);
        message.coin !== undefined &&
            (obj.coin = message.coin ? Coin.toJSON(message.coin) : undefined);
        message.lockupPeriod !== undefined &&
            (obj.lockupPeriod = lockupTypesToJSON(message.lockupPeriod));
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
        if (object.coin !== undefined && object.coin !== null) {
            message.coin = Coin.fromPartial(object.coin);
        }
        else {
            message.coin = undefined;
        }
        if (object.lockupPeriod !== undefined && object.lockupPeriod !== null) {
            message.lockupPeriod = object.lockupPeriod;
        }
        else {
            message.lockupPeriod = 0;
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
const baseMsgRequestWithdraw = {
    creator: "",
    riskProfile: "",
    vaultID: "",
};
export const MsgRequestWithdraw = {
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
        if (message.coin !== undefined) {
            Coin.encode(message.coin, writer.uint32(34).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgRequestWithdraw };
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
                    message.coin = Coin.decode(reader, reader.uint32());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgRequestWithdraw };
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
        if (object.coin !== undefined && object.coin !== null) {
            message.coin = Coin.fromJSON(object.coin);
        }
        else {
            message.coin = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.riskProfile !== undefined &&
            (obj.riskProfile = message.riskProfile);
        message.vaultID !== undefined && (obj.vaultID = message.vaultID);
        message.coin !== undefined &&
            (obj.coin = message.coin ? Coin.toJSON(message.coin) : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgRequestWithdraw };
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
        if (object.coin !== undefined && object.coin !== null) {
            message.coin = Coin.fromPartial(object.coin);
        }
        else {
            message.coin = undefined;
        }
        return message;
    },
};
const baseMsgRequestWithdrawResponse = {};
export const MsgRequestWithdrawResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgRequestWithdrawResponse,
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
            ...baseMsgRequestWithdrawResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgRequestWithdrawResponse,
        };
        return message;
    },
};
const baseMsgClaimRewards = { creator: "", vaultID: "" };
export const MsgClaimRewards = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.vaultID !== "") {
            writer.uint32(18).string(message.vaultID);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgClaimRewards };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.vaultID = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgClaimRewards };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.vaultID !== undefined && object.vaultID !== null) {
            message.vaultID = String(object.vaultID);
        }
        else {
            message.vaultID = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.vaultID !== undefined && (obj.vaultID = message.vaultID);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgClaimRewards };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.vaultID !== undefined && object.vaultID !== null) {
            message.vaultID = object.vaultID;
        }
        else {
            message.vaultID = "";
        }
        return message;
    },
};
const baseMsgClaimRewardsResponse = {};
export const MsgClaimRewardsResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgClaimRewardsResponse,
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
            ...baseMsgClaimRewardsResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgClaimRewardsResponse,
        };
        return message;
    },
};
const baseMsgRequestWithdrawAll = { creator: "", vaultID: "" };
export const MsgRequestWithdrawAll = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.vaultID !== "") {
            writer.uint32(18).string(message.vaultID);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgRequestWithdrawAll };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.vaultID = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgRequestWithdrawAll };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.vaultID !== undefined && object.vaultID !== null) {
            message.vaultID = String(object.vaultID);
        }
        else {
            message.vaultID = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.vaultID !== undefined && (obj.vaultID = message.vaultID);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgRequestWithdrawAll };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.vaultID !== undefined && object.vaultID !== null) {
            message.vaultID = object.vaultID;
        }
        else {
            message.vaultID = "";
        }
        return message;
    },
};
const baseMsgRequestWithdrawAllResponse = {};
export const MsgRequestWithdrawAllResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgRequestWithdrawAllResponse,
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
            ...baseMsgRequestWithdrawAllResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgRequestWithdrawAllResponse,
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
    RequestWithdraw(request) {
        const data = MsgRequestWithdraw.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Msg", "RequestWithdraw", data);
        return promise.then((data) => MsgRequestWithdrawResponse.decode(new Reader(data)));
    }
    ClaimRewards(request) {
        const data = MsgClaimRewards.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Msg", "ClaimRewards", data);
        return promise.then((data) => MsgClaimRewardsResponse.decode(new Reader(data)));
    }
    RequestWithdrawAll(request) {
        const data = MsgRequestWithdrawAll.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Msg", "RequestWithdrawAll", data);
        return promise.then((data) => MsgRequestWithdrawAllResponse.decode(new Reader(data)));
    }
}
