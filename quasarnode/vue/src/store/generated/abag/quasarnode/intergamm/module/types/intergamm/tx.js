/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { BalancerPoolParams } from "../osmosis/gamm/pool-models/balancer/balancerPool";
import { PoolAsset } from "../osmosis/gamm/v1beta1/pool";
import { Coin } from "../cosmos/base/v1beta1/coin";
export const protobufPackage = "abag.quasarnode.intergamm.intergamm";
const baseMsgSendIbcCreatePool = {
    creator: "",
    connectionId: "",
    timeoutTimestamp: 0,
    futurePoolGovernor: "",
};
export const MsgSendIbcCreatePool = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.connectionId !== "") {
            writer.uint32(18).string(message.connectionId);
        }
        if (message.timeoutTimestamp !== 0) {
            writer.uint32(24).uint64(message.timeoutTimestamp);
        }
        if (message.poolParams !== undefined) {
            BalancerPoolParams.encode(message.poolParams, writer.uint32(34).fork()).ldelim();
        }
        for (const v of message.poolAssets) {
            PoolAsset.encode(v, writer.uint32(42).fork()).ldelim();
        }
        if (message.futurePoolGovernor !== "") {
            writer.uint32(58).string(message.futurePoolGovernor);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgSendIbcCreatePool };
        message.poolAssets = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.connectionId = reader.string();
                    break;
                case 3:
                    message.timeoutTimestamp = longToNumber(reader.uint64());
                    break;
                case 4:
                    message.poolParams = BalancerPoolParams.decode(reader, reader.uint32());
                    break;
                case 5:
                    message.poolAssets.push(PoolAsset.decode(reader, reader.uint32()));
                    break;
                case 7:
                    message.futurePoolGovernor = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgSendIbcCreatePool };
        message.poolAssets = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.connectionId !== undefined && object.connectionId !== null) {
            message.connectionId = String(object.connectionId);
        }
        else {
            message.connectionId = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = Number(object.timeoutTimestamp);
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.poolParams !== undefined && object.poolParams !== null) {
            message.poolParams = BalancerPoolParams.fromJSON(object.poolParams);
        }
        else {
            message.poolParams = undefined;
        }
        if (object.poolAssets !== undefined && object.poolAssets !== null) {
            for (const e of object.poolAssets) {
                message.poolAssets.push(PoolAsset.fromJSON(e));
            }
        }
        if (object.futurePoolGovernor !== undefined &&
            object.futurePoolGovernor !== null) {
            message.futurePoolGovernor = String(object.futurePoolGovernor);
        }
        else {
            message.futurePoolGovernor = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.connectionId !== undefined &&
            (obj.connectionId = message.connectionId);
        message.timeoutTimestamp !== undefined &&
            (obj.timeoutTimestamp = message.timeoutTimestamp);
        message.poolParams !== undefined &&
            (obj.poolParams = message.poolParams
                ? BalancerPoolParams.toJSON(message.poolParams)
                : undefined);
        if (message.poolAssets) {
            obj.poolAssets = message.poolAssets.map((e) => e ? PoolAsset.toJSON(e) : undefined);
        }
        else {
            obj.poolAssets = [];
        }
        message.futurePoolGovernor !== undefined &&
            (obj.futurePoolGovernor = message.futurePoolGovernor);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgSendIbcCreatePool };
        message.poolAssets = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.connectionId !== undefined && object.connectionId !== null) {
            message.connectionId = object.connectionId;
        }
        else {
            message.connectionId = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = object.timeoutTimestamp;
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.poolParams !== undefined && object.poolParams !== null) {
            message.poolParams = BalancerPoolParams.fromPartial(object.poolParams);
        }
        else {
            message.poolParams = undefined;
        }
        if (object.poolAssets !== undefined && object.poolAssets !== null) {
            for (const e of object.poolAssets) {
                message.poolAssets.push(PoolAsset.fromPartial(e));
            }
        }
        if (object.futurePoolGovernor !== undefined &&
            object.futurePoolGovernor !== null) {
            message.futurePoolGovernor = object.futurePoolGovernor;
        }
        else {
            message.futurePoolGovernor = "";
        }
        return message;
    },
};
const baseMsgSendIbcCreatePoolResponse = {};
export const MsgSendIbcCreatePoolResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgSendIbcCreatePoolResponse,
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
            ...baseMsgSendIbcCreatePoolResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgSendIbcCreatePoolResponse,
        };
        return message;
    },
};
const baseMsgSendIbcJoinPool = {
    creator: "",
    port: "",
    channelID: "",
    timeoutTimestamp: 0,
    poolId: 0,
    shareOutAmount: "",
};
export const MsgSendIbcJoinPool = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.port !== "") {
            writer.uint32(18).string(message.port);
        }
        if (message.channelID !== "") {
            writer.uint32(26).string(message.channelID);
        }
        if (message.timeoutTimestamp !== 0) {
            writer.uint32(32).uint64(message.timeoutTimestamp);
        }
        if (message.poolId !== 0) {
            writer.uint32(40).uint64(message.poolId);
        }
        if (message.shareOutAmount !== "") {
            writer.uint32(50).string(message.shareOutAmount);
        }
        for (const v of message.tokenInMaxs) {
            Coin.encode(v, writer.uint32(58).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgSendIbcJoinPool };
        message.tokenInMaxs = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.port = reader.string();
                    break;
                case 3:
                    message.channelID = reader.string();
                    break;
                case 4:
                    message.timeoutTimestamp = longToNumber(reader.uint64());
                    break;
                case 5:
                    message.poolId = longToNumber(reader.uint64());
                    break;
                case 6:
                    message.shareOutAmount = reader.string();
                    break;
                case 7:
                    message.tokenInMaxs.push(Coin.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgSendIbcJoinPool };
        message.tokenInMaxs = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.port !== undefined && object.port !== null) {
            message.port = String(object.port);
        }
        else {
            message.port = "";
        }
        if (object.channelID !== undefined && object.channelID !== null) {
            message.channelID = String(object.channelID);
        }
        else {
            message.channelID = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = Number(object.timeoutTimestamp);
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = Number(object.poolId);
        }
        else {
            message.poolId = 0;
        }
        if (object.shareOutAmount !== undefined && object.shareOutAmount !== null) {
            message.shareOutAmount = String(object.shareOutAmount);
        }
        else {
            message.shareOutAmount = "";
        }
        if (object.tokenInMaxs !== undefined && object.tokenInMaxs !== null) {
            for (const e of object.tokenInMaxs) {
                message.tokenInMaxs.push(Coin.fromJSON(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.port !== undefined && (obj.port = message.port);
        message.channelID !== undefined && (obj.channelID = message.channelID);
        message.timeoutTimestamp !== undefined &&
            (obj.timeoutTimestamp = message.timeoutTimestamp);
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.shareOutAmount !== undefined &&
            (obj.shareOutAmount = message.shareOutAmount);
        if (message.tokenInMaxs) {
            obj.tokenInMaxs = message.tokenInMaxs.map((e) => e ? Coin.toJSON(e) : undefined);
        }
        else {
            obj.tokenInMaxs = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgSendIbcJoinPool };
        message.tokenInMaxs = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.port !== undefined && object.port !== null) {
            message.port = object.port;
        }
        else {
            message.port = "";
        }
        if (object.channelID !== undefined && object.channelID !== null) {
            message.channelID = object.channelID;
        }
        else {
            message.channelID = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = object.timeoutTimestamp;
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = 0;
        }
        if (object.shareOutAmount !== undefined && object.shareOutAmount !== null) {
            message.shareOutAmount = object.shareOutAmount;
        }
        else {
            message.shareOutAmount = "";
        }
        if (object.tokenInMaxs !== undefined && object.tokenInMaxs !== null) {
            for (const e of object.tokenInMaxs) {
                message.tokenInMaxs.push(Coin.fromPartial(e));
            }
        }
        return message;
    },
};
const baseMsgSendIbcJoinPoolResponse = {};
export const MsgSendIbcJoinPoolResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgSendIbcJoinPoolResponse,
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
            ...baseMsgSendIbcJoinPoolResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgSendIbcJoinPoolResponse,
        };
        return message;
    },
};
const baseMsgSendIbcExitPool = {
    creator: "",
    port: "",
    channelID: "",
    timeoutTimestamp: 0,
    poolId: 0,
    shareInAmount: "",
};
export const MsgSendIbcExitPool = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.port !== "") {
            writer.uint32(18).string(message.port);
        }
        if (message.channelID !== "") {
            writer.uint32(26).string(message.channelID);
        }
        if (message.timeoutTimestamp !== 0) {
            writer.uint32(32).uint64(message.timeoutTimestamp);
        }
        if (message.poolId !== 0) {
            writer.uint32(40).uint64(message.poolId);
        }
        if (message.shareInAmount !== "") {
            writer.uint32(50).string(message.shareInAmount);
        }
        for (const v of message.tokenOutMins) {
            Coin.encode(v, writer.uint32(58).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgSendIbcExitPool };
        message.tokenOutMins = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.port = reader.string();
                    break;
                case 3:
                    message.channelID = reader.string();
                    break;
                case 4:
                    message.timeoutTimestamp = longToNumber(reader.uint64());
                    break;
                case 5:
                    message.poolId = longToNumber(reader.uint64());
                    break;
                case 6:
                    message.shareInAmount = reader.string();
                    break;
                case 7:
                    message.tokenOutMins.push(Coin.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgSendIbcExitPool };
        message.tokenOutMins = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.port !== undefined && object.port !== null) {
            message.port = String(object.port);
        }
        else {
            message.port = "";
        }
        if (object.channelID !== undefined && object.channelID !== null) {
            message.channelID = String(object.channelID);
        }
        else {
            message.channelID = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = Number(object.timeoutTimestamp);
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = Number(object.poolId);
        }
        else {
            message.poolId = 0;
        }
        if (object.shareInAmount !== undefined && object.shareInAmount !== null) {
            message.shareInAmount = String(object.shareInAmount);
        }
        else {
            message.shareInAmount = "";
        }
        if (object.tokenOutMins !== undefined && object.tokenOutMins !== null) {
            for (const e of object.tokenOutMins) {
                message.tokenOutMins.push(Coin.fromJSON(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.port !== undefined && (obj.port = message.port);
        message.channelID !== undefined && (obj.channelID = message.channelID);
        message.timeoutTimestamp !== undefined &&
            (obj.timeoutTimestamp = message.timeoutTimestamp);
        message.poolId !== undefined && (obj.poolId = message.poolId);
        message.shareInAmount !== undefined &&
            (obj.shareInAmount = message.shareInAmount);
        if (message.tokenOutMins) {
            obj.tokenOutMins = message.tokenOutMins.map((e) => e ? Coin.toJSON(e) : undefined);
        }
        else {
            obj.tokenOutMins = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgSendIbcExitPool };
        message.tokenOutMins = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.port !== undefined && object.port !== null) {
            message.port = object.port;
        }
        else {
            message.port = "";
        }
        if (object.channelID !== undefined && object.channelID !== null) {
            message.channelID = object.channelID;
        }
        else {
            message.channelID = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = object.timeoutTimestamp;
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.poolId !== undefined && object.poolId !== null) {
            message.poolId = object.poolId;
        }
        else {
            message.poolId = 0;
        }
        if (object.shareInAmount !== undefined && object.shareInAmount !== null) {
            message.shareInAmount = object.shareInAmount;
        }
        else {
            message.shareInAmount = "";
        }
        if (object.tokenOutMins !== undefined && object.tokenOutMins !== null) {
            for (const e of object.tokenOutMins) {
                message.tokenOutMins.push(Coin.fromPartial(e));
            }
        }
        return message;
    },
};
const baseMsgSendIbcExitPoolResponse = {};
export const MsgSendIbcExitPoolResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgSendIbcExitPoolResponse,
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
            ...baseMsgSendIbcExitPoolResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgSendIbcExitPoolResponse,
        };
        return message;
    },
};
const baseMsgSendIbcWithdraw = {
    creator: "",
    port: "",
    channelID: "",
    timeoutTimestamp: 0,
    transferPort: "",
    transferChannel: "",
    receiver: "",
};
export const MsgSendIbcWithdraw = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.port !== "") {
            writer.uint32(18).string(message.port);
        }
        if (message.channelID !== "") {
            writer.uint32(26).string(message.channelID);
        }
        if (message.timeoutTimestamp !== 0) {
            writer.uint32(32).uint64(message.timeoutTimestamp);
        }
        if (message.transferPort !== "") {
            writer.uint32(42).string(message.transferPort);
        }
        if (message.transferChannel !== "") {
            writer.uint32(50).string(message.transferChannel);
        }
        if (message.receiver !== "") {
            writer.uint32(58).string(message.receiver);
        }
        for (const v of message.assets) {
            Coin.encode(v, writer.uint32(66).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgSendIbcWithdraw };
        message.assets = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.port = reader.string();
                    break;
                case 3:
                    message.channelID = reader.string();
                    break;
                case 4:
                    message.timeoutTimestamp = longToNumber(reader.uint64());
                    break;
                case 5:
                    message.transferPort = reader.string();
                    break;
                case 6:
                    message.transferChannel = reader.string();
                    break;
                case 7:
                    message.receiver = reader.string();
                    break;
                case 8:
                    message.assets.push(Coin.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgSendIbcWithdraw };
        message.assets = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.port !== undefined && object.port !== null) {
            message.port = String(object.port);
        }
        else {
            message.port = "";
        }
        if (object.channelID !== undefined && object.channelID !== null) {
            message.channelID = String(object.channelID);
        }
        else {
            message.channelID = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = Number(object.timeoutTimestamp);
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.transferPort !== undefined && object.transferPort !== null) {
            message.transferPort = String(object.transferPort);
        }
        else {
            message.transferPort = "";
        }
        if (object.transferChannel !== undefined &&
            object.transferChannel !== null) {
            message.transferChannel = String(object.transferChannel);
        }
        else {
            message.transferChannel = "";
        }
        if (object.receiver !== undefined && object.receiver !== null) {
            message.receiver = String(object.receiver);
        }
        else {
            message.receiver = "";
        }
        if (object.assets !== undefined && object.assets !== null) {
            for (const e of object.assets) {
                message.assets.push(Coin.fromJSON(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.port !== undefined && (obj.port = message.port);
        message.channelID !== undefined && (obj.channelID = message.channelID);
        message.timeoutTimestamp !== undefined &&
            (obj.timeoutTimestamp = message.timeoutTimestamp);
        message.transferPort !== undefined &&
            (obj.transferPort = message.transferPort);
        message.transferChannel !== undefined &&
            (obj.transferChannel = message.transferChannel);
        message.receiver !== undefined && (obj.receiver = message.receiver);
        if (message.assets) {
            obj.assets = message.assets.map((e) => (e ? Coin.toJSON(e) : undefined));
        }
        else {
            obj.assets = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgSendIbcWithdraw };
        message.assets = [];
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.port !== undefined && object.port !== null) {
            message.port = object.port;
        }
        else {
            message.port = "";
        }
        if (object.channelID !== undefined && object.channelID !== null) {
            message.channelID = object.channelID;
        }
        else {
            message.channelID = "";
        }
        if (object.timeoutTimestamp !== undefined &&
            object.timeoutTimestamp !== null) {
            message.timeoutTimestamp = object.timeoutTimestamp;
        }
        else {
            message.timeoutTimestamp = 0;
        }
        if (object.transferPort !== undefined && object.transferPort !== null) {
            message.transferPort = object.transferPort;
        }
        else {
            message.transferPort = "";
        }
        if (object.transferChannel !== undefined &&
            object.transferChannel !== null) {
            message.transferChannel = object.transferChannel;
        }
        else {
            message.transferChannel = "";
        }
        if (object.receiver !== undefined && object.receiver !== null) {
            message.receiver = object.receiver;
        }
        else {
            message.receiver = "";
        }
        if (object.assets !== undefined && object.assets !== null) {
            for (const e of object.assets) {
                message.assets.push(Coin.fromPartial(e));
            }
        }
        return message;
    },
};
const baseMsgSendIbcWithdrawResponse = {};
export const MsgSendIbcWithdrawResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgSendIbcWithdrawResponse,
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
            ...baseMsgSendIbcWithdrawResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgSendIbcWithdrawResponse,
        };
        return message;
    },
};
const baseMsgRegisterAccount = { creator: "", connectionId: "" };
export const MsgRegisterAccount = {
    encode(message, writer = Writer.create()) {
        if (message.creator !== "") {
            writer.uint32(10).string(message.creator);
        }
        if (message.connectionId !== "") {
            writer.uint32(18).string(message.connectionId);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseMsgRegisterAccount };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.creator = reader.string();
                    break;
                case 2:
                    message.connectionId = reader.string();
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseMsgRegisterAccount };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = String(object.creator);
        }
        else {
            message.creator = "";
        }
        if (object.connectionId !== undefined && object.connectionId !== null) {
            message.connectionId = String(object.connectionId);
        }
        else {
            message.connectionId = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.creator !== undefined && (obj.creator = message.creator);
        message.connectionId !== undefined &&
            (obj.connectionId = message.connectionId);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseMsgRegisterAccount };
        if (object.creator !== undefined && object.creator !== null) {
            message.creator = object.creator;
        }
        else {
            message.creator = "";
        }
        if (object.connectionId !== undefined && object.connectionId !== null) {
            message.connectionId = object.connectionId;
        }
        else {
            message.connectionId = "";
        }
        return message;
    },
};
const baseMsgRegisterAccountResponse = {};
export const MsgRegisterAccountResponse = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseMsgRegisterAccountResponse,
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
            ...baseMsgRegisterAccountResponse,
        };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = {
            ...baseMsgRegisterAccountResponse,
        };
        return message;
    },
};
export class MsgClientImpl {
    constructor(rpc) {
        this.rpc = rpc;
    }
    SendIbcCreatePool(request) {
        const data = MsgSendIbcCreatePool.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.intergamm.intergamm.Msg", "SendIbcCreatePool", data);
        return promise.then((data) => MsgSendIbcCreatePoolResponse.decode(new Reader(data)));
    }
    SendIbcJoinPool(request) {
        const data = MsgSendIbcJoinPool.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.intergamm.intergamm.Msg", "SendIbcJoinPool", data);
        return promise.then((data) => MsgSendIbcJoinPoolResponse.decode(new Reader(data)));
    }
    SendIbcExitPool(request) {
        const data = MsgSendIbcExitPool.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.intergamm.intergamm.Msg", "SendIbcExitPool", data);
        return promise.then((data) => MsgSendIbcExitPoolResponse.decode(new Reader(data)));
    }
    SendIbcWithdraw(request) {
        const data = MsgSendIbcWithdraw.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.intergamm.intergamm.Msg", "SendIbcWithdraw", data);
        return promise.then((data) => MsgSendIbcWithdrawResponse.decode(new Reader(data)));
    }
    RegisterAccount(request) {
        const data = MsgRegisterAccount.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.intergamm.intergamm.Msg", "RegisterAccount", data);
        return promise.then((data) => MsgRegisterAccountResponse.decode(new Reader(data)));
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
