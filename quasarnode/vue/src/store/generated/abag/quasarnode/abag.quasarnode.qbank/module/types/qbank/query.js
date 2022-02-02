/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { Params } from "../qbank/params";
import { Deposit } from "../qbank/deposit";
import { PageRequest, PageResponse, } from "../cosmos/base/query/v1beta1/pagination";
import { Withdraw } from "../qbank/withdraw";
import { FeeData } from "../qbank/fee_data";
import { QCoins } from "../qbank/common";
export const protobufPackage = "abag.quasarnode.qbank";
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
const baseQueryGetDepositRequest = { id: 0 };
export const QueryGetDepositRequest = {
    encode(message, writer = Writer.create()) {
        if (message.id !== 0) {
            writer.uint32(8).uint64(message.id);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseQueryGetDepositRequest };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.id = longToNumber(reader.uint64());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseQueryGetDepositRequest };
        if (object.id !== undefined && object.id !== null) {
            message.id = Number(object.id);
        }
        else {
            message.id = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.id !== undefined && (obj.id = message.id);
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseQueryGetDepositRequest };
        if (object.id !== undefined && object.id !== null) {
            message.id = object.id;
        }
        else {
            message.id = 0;
        }
        return message;
    },
};
const baseQueryGetDepositResponse = {};
export const QueryGetDepositResponse = {
    encode(message, writer = Writer.create()) {
        if (message.Deposit !== undefined) {
            Deposit.encode(message.Deposit, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetDepositResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.Deposit = Deposit.decode(reader, reader.uint32());
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
            ...baseQueryGetDepositResponse,
        };
        if (object.Deposit !== undefined && object.Deposit !== null) {
            message.Deposit = Deposit.fromJSON(object.Deposit);
        }
        else {
            message.Deposit = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.Deposit !== undefined &&
            (obj.Deposit = message.Deposit
                ? Deposit.toJSON(message.Deposit)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetDepositResponse,
        };
        if (object.Deposit !== undefined && object.Deposit !== null) {
            message.Deposit = Deposit.fromPartial(object.Deposit);
        }
        else {
            message.Deposit = undefined;
        }
        return message;
    },
};
const baseQueryAllDepositRequest = {};
export const QueryAllDepositRequest = {
    encode(message, writer = Writer.create()) {
        if (message.pagination !== undefined) {
            PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseQueryAllDepositRequest };
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
        const message = { ...baseQueryAllDepositRequest };
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
        const message = { ...baseQueryAllDepositRequest };
        if (object.pagination !== undefined && object.pagination !== null) {
            message.pagination = PageRequest.fromPartial(object.pagination);
        }
        else {
            message.pagination = undefined;
        }
        return message;
    },
};
const baseQueryAllDepositResponse = {};
export const QueryAllDepositResponse = {
    encode(message, writer = Writer.create()) {
        for (const v of message.Deposit) {
            Deposit.encode(v, writer.uint32(10).fork()).ldelim();
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
            ...baseQueryAllDepositResponse,
        };
        message.Deposit = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.Deposit.push(Deposit.decode(reader, reader.uint32()));
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
            ...baseQueryAllDepositResponse,
        };
        message.Deposit = [];
        if (object.Deposit !== undefined && object.Deposit !== null) {
            for (const e of object.Deposit) {
                message.Deposit.push(Deposit.fromJSON(e));
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
        if (message.Deposit) {
            obj.Deposit = message.Deposit.map((e) => e ? Deposit.toJSON(e) : undefined);
        }
        else {
            obj.Deposit = [];
        }
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageResponse.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllDepositResponse,
        };
        message.Deposit = [];
        if (object.Deposit !== undefined && object.Deposit !== null) {
            for (const e of object.Deposit) {
                message.Deposit.push(Deposit.fromPartial(e));
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
const baseQueryUserDenomDepositRequest = { userAcc: "", denom: "" };
export const QueryUserDenomDepositRequest = {
    encode(message, writer = Writer.create()) {
        if (message.userAcc !== "") {
            writer.uint32(10).string(message.userAcc);
        }
        if (message.denom !== "") {
            writer.uint32(18).string(message.denom);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDenomDepositRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.userAcc = reader.string();
                    break;
                case 2:
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
        const message = {
            ...baseQueryUserDenomDepositRequest,
        };
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = String(object.userAcc);
        }
        else {
            message.userAcc = "";
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
        message.userAcc !== undefined && (obj.userAcc = message.userAcc);
        message.denom !== undefined && (obj.denom = message.denom);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDenomDepositRequest,
        };
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = object.userAcc;
        }
        else {
            message.userAcc = "";
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
const baseQueryUserDenomDepositResponse = { amount: 0 };
export const QueryUserDenomDepositResponse = {
    encode(message, writer = Writer.create()) {
        if (message.amount !== 0) {
            writer.uint32(8).uint64(message.amount);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDenomDepositResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.amount = longToNumber(reader.uint64());
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
            ...baseQueryUserDenomDepositResponse,
        };
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = Number(object.amount);
        }
        else {
            message.amount = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.amount !== undefined && (obj.amount = message.amount);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDenomDepositResponse,
        };
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = object.amount;
        }
        else {
            message.amount = 0;
        }
        return message;
    },
};
const baseQueryGetWithdrawRequest = { id: 0 };
export const QueryGetWithdrawRequest = {
    encode(message, writer = Writer.create()) {
        if (message.id !== 0) {
            writer.uint32(8).uint64(message.id);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetWithdrawRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.id = longToNumber(reader.uint64());
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
            ...baseQueryGetWithdrawRequest,
        };
        if (object.id !== undefined && object.id !== null) {
            message.id = Number(object.id);
        }
        else {
            message.id = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.id !== undefined && (obj.id = message.id);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetWithdrawRequest,
        };
        if (object.id !== undefined && object.id !== null) {
            message.id = object.id;
        }
        else {
            message.id = 0;
        }
        return message;
    },
};
const baseQueryGetWithdrawResponse = {};
export const QueryGetWithdrawResponse = {
    encode(message, writer = Writer.create()) {
        if (message.Withdraw !== undefined) {
            Withdraw.encode(message.Withdraw, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetWithdrawResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.Withdraw = Withdraw.decode(reader, reader.uint32());
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
            ...baseQueryGetWithdrawResponse,
        };
        if (object.Withdraw !== undefined && object.Withdraw !== null) {
            message.Withdraw = Withdraw.fromJSON(object.Withdraw);
        }
        else {
            message.Withdraw = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.Withdraw !== undefined &&
            (obj.Withdraw = message.Withdraw
                ? Withdraw.toJSON(message.Withdraw)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetWithdrawResponse,
        };
        if (object.Withdraw !== undefined && object.Withdraw !== null) {
            message.Withdraw = Withdraw.fromPartial(object.Withdraw);
        }
        else {
            message.Withdraw = undefined;
        }
        return message;
    },
};
const baseQueryAllWithdrawRequest = {};
export const QueryAllWithdrawRequest = {
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
            ...baseQueryAllWithdrawRequest,
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
            ...baseQueryAllWithdrawRequest,
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
            ...baseQueryAllWithdrawRequest,
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
const baseQueryAllWithdrawResponse = {};
export const QueryAllWithdrawResponse = {
    encode(message, writer = Writer.create()) {
        for (const v of message.Withdraw) {
            Withdraw.encode(v, writer.uint32(10).fork()).ldelim();
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
            ...baseQueryAllWithdrawResponse,
        };
        message.Withdraw = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.Withdraw.push(Withdraw.decode(reader, reader.uint32()));
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
            ...baseQueryAllWithdrawResponse,
        };
        message.Withdraw = [];
        if (object.Withdraw !== undefined && object.Withdraw !== null) {
            for (const e of object.Withdraw) {
                message.Withdraw.push(Withdraw.fromJSON(e));
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
        if (message.Withdraw) {
            obj.Withdraw = message.Withdraw.map((e) => e ? Withdraw.toJSON(e) : undefined);
        }
        else {
            obj.Withdraw = [];
        }
        message.pagination !== undefined &&
            (obj.pagination = message.pagination
                ? PageResponse.toJSON(message.pagination)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryAllWithdrawResponse,
        };
        message.Withdraw = [];
        if (object.Withdraw !== undefined && object.Withdraw !== null) {
            for (const e of object.Withdraw) {
                message.Withdraw.push(Withdraw.fromPartial(e));
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
const baseQueryGetFeeDataRequest = {};
export const QueryGetFeeDataRequest = {
    encode(_, writer = Writer.create()) {
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseQueryGetFeeDataRequest };
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
        const message = { ...baseQueryGetFeeDataRequest };
        return message;
    },
    toJSON(_) {
        const obj = {};
        return obj;
    },
    fromPartial(_) {
        const message = { ...baseQueryGetFeeDataRequest };
        return message;
    },
};
const baseQueryGetFeeDataResponse = {};
export const QueryGetFeeDataResponse = {
    encode(message, writer = Writer.create()) {
        if (message.FeeData !== undefined) {
            FeeData.encode(message.FeeData, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetFeeDataResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.FeeData = FeeData.decode(reader, reader.uint32());
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
            ...baseQueryGetFeeDataResponse,
        };
        if (object.FeeData !== undefined && object.FeeData !== null) {
            message.FeeData = FeeData.fromJSON(object.FeeData);
        }
        else {
            message.FeeData = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.FeeData !== undefined &&
            (obj.FeeData = message.FeeData
                ? FeeData.toJSON(message.FeeData)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetFeeDataResponse,
        };
        if (object.FeeData !== undefined && object.FeeData !== null) {
            message.FeeData = FeeData.fromPartial(object.FeeData);
        }
        else {
            message.FeeData = undefined;
        }
        return message;
    },
};
const baseQueryUserDepositRequest = { userAcc: "" };
export const QueryUserDepositRequest = {
    encode(message, writer = Writer.create()) {
        if (message.userAcc !== "") {
            writer.uint32(10).string(message.userAcc);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDepositRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.userAcc = reader.string();
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
            ...baseQueryUserDepositRequest,
        };
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = String(object.userAcc);
        }
        else {
            message.userAcc = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.userAcc !== undefined && (obj.userAcc = message.userAcc);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDepositRequest,
        };
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = object.userAcc;
        }
        else {
            message.userAcc = "";
        }
        return message;
    },
};
const baseQueryUserDepositResponse = {};
export const QueryUserDepositResponse = {
    encode(message, writer = Writer.create()) {
        if (message.coins !== undefined) {
            QCoins.encode(message.coins, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDepositResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.coins = QCoins.decode(reader, reader.uint32());
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
            ...baseQueryUserDepositResponse,
        };
        if (object.coins !== undefined && object.coins !== null) {
            message.coins = QCoins.fromJSON(object.coins);
        }
        else {
            message.coins = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.coins !== undefined &&
            (obj.coins = message.coins ? QCoins.toJSON(message.coins) : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDepositResponse,
        };
        if (object.coins !== undefined && object.coins !== null) {
            message.coins = QCoins.fromPartial(object.coins);
        }
        else {
            message.coins = undefined;
        }
        return message;
    },
};
const baseQueryUserDenomLockupDepositRequest = {
    userAcc: "",
    denom: "",
    lockupType: "",
};
export const QueryUserDenomLockupDepositRequest = {
    encode(message, writer = Writer.create()) {
        if (message.userAcc !== "") {
            writer.uint32(10).string(message.userAcc);
        }
        if (message.denom !== "") {
            writer.uint32(18).string(message.denom);
        }
        if (message.lockupType !== "") {
            writer.uint32(26).string(message.lockupType);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDenomLockupDepositRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.userAcc = reader.string();
                    break;
                case 2:
                    message.denom = reader.string();
                    break;
                case 3:
                    message.lockupType = reader.string();
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
            ...baseQueryUserDenomLockupDepositRequest,
        };
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = String(object.userAcc);
        }
        else {
            message.userAcc = "";
        }
        if (object.denom !== undefined && object.denom !== null) {
            message.denom = String(object.denom);
        }
        else {
            message.denom = "";
        }
        if (object.lockupType !== undefined && object.lockupType !== null) {
            message.lockupType = String(object.lockupType);
        }
        else {
            message.lockupType = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.userAcc !== undefined && (obj.userAcc = message.userAcc);
        message.denom !== undefined && (obj.denom = message.denom);
        message.lockupType !== undefined && (obj.lockupType = message.lockupType);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDenomLockupDepositRequest,
        };
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = object.userAcc;
        }
        else {
            message.userAcc = "";
        }
        if (object.denom !== undefined && object.denom !== null) {
            message.denom = object.denom;
        }
        else {
            message.denom = "";
        }
        if (object.lockupType !== undefined && object.lockupType !== null) {
            message.lockupType = object.lockupType;
        }
        else {
            message.lockupType = "";
        }
        return message;
    },
};
const baseQueryUserDenomLockupDepositResponse = { amount: 0 };
export const QueryUserDenomLockupDepositResponse = {
    encode(message, writer = Writer.create()) {
        if (message.amount !== 0) {
            writer.uint32(8).uint64(message.amount);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDenomLockupDepositResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.amount = longToNumber(reader.uint64());
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
            ...baseQueryUserDenomLockupDepositResponse,
        };
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = Number(object.amount);
        }
        else {
            message.amount = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.amount !== undefined && (obj.amount = message.amount);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDenomLockupDepositResponse,
        };
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = object.amount;
        }
        else {
            message.amount = 0;
        }
        return message;
    },
};
const baseQueryUserDenomEpochLockupDepositRequest = {
    userAcc: "",
    denom: "",
    epochDay: 0,
    lockupType: "",
};
export const QueryUserDenomEpochLockupDepositRequest = {
    encode(message, writer = Writer.create()) {
        if (message.userAcc !== "") {
            writer.uint32(10).string(message.userAcc);
        }
        if (message.denom !== "") {
            writer.uint32(18).string(message.denom);
        }
        writer.uint32(26).fork();
        for (const v of message.epochDay) {
            writer.uint64(v);
        }
        writer.ldelim();
        if (message.lockupType !== "") {
            writer.uint32(34).string(message.lockupType);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDenomEpochLockupDepositRequest,
        };
        message.epochDay = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.userAcc = reader.string();
                    break;
                case 2:
                    message.denom = reader.string();
                    break;
                case 3:
                    if ((tag & 7) === 2) {
                        const end2 = reader.uint32() + reader.pos;
                        while (reader.pos < end2) {
                            message.epochDay.push(longToNumber(reader.uint64()));
                        }
                    }
                    else {
                        message.epochDay.push(longToNumber(reader.uint64()));
                    }
                    break;
                case 4:
                    message.lockupType = reader.string();
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
            ...baseQueryUserDenomEpochLockupDepositRequest,
        };
        message.epochDay = [];
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = String(object.userAcc);
        }
        else {
            message.userAcc = "";
        }
        if (object.denom !== undefined && object.denom !== null) {
            message.denom = String(object.denom);
        }
        else {
            message.denom = "";
        }
        if (object.epochDay !== undefined && object.epochDay !== null) {
            for (const e of object.epochDay) {
                message.epochDay.push(Number(e));
            }
        }
        if (object.lockupType !== undefined && object.lockupType !== null) {
            message.lockupType = String(object.lockupType);
        }
        else {
            message.lockupType = "";
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.userAcc !== undefined && (obj.userAcc = message.userAcc);
        message.denom !== undefined && (obj.denom = message.denom);
        if (message.epochDay) {
            obj.epochDay = message.epochDay.map((e) => e);
        }
        else {
            obj.epochDay = [];
        }
        message.lockupType !== undefined && (obj.lockupType = message.lockupType);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDenomEpochLockupDepositRequest,
        };
        message.epochDay = [];
        if (object.userAcc !== undefined && object.userAcc !== null) {
            message.userAcc = object.userAcc;
        }
        else {
            message.userAcc = "";
        }
        if (object.denom !== undefined && object.denom !== null) {
            message.denom = object.denom;
        }
        else {
            message.denom = "";
        }
        if (object.epochDay !== undefined && object.epochDay !== null) {
            for (const e of object.epochDay) {
                message.epochDay.push(e);
            }
        }
        if (object.lockupType !== undefined && object.lockupType !== null) {
            message.lockupType = object.lockupType;
        }
        else {
            message.lockupType = "";
        }
        return message;
    },
};
const baseQueryUserDenomEpochLockupDepositResponse = { amount: 0 };
export const QueryUserDenomEpochLockupDepositResponse = {
    encode(message, writer = Writer.create()) {
        if (message.amount !== 0) {
            writer.uint32(8).uint64(message.amount);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryUserDenomEpochLockupDepositResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.amount = longToNumber(reader.uint64());
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
            ...baseQueryUserDenomEpochLockupDepositResponse,
        };
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = Number(object.amount);
        }
        else {
            message.amount = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.amount !== undefined && (obj.amount = message.amount);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryUserDenomEpochLockupDepositResponse,
        };
        if (object.amount !== undefined && object.amount !== null) {
            message.amount = object.amount;
        }
        else {
            message.amount = 0;
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
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "Params", data);
        return promise.then((data) => QueryParamsResponse.decode(new Reader(data)));
    }
    Deposit(request) {
        const data = QueryGetDepositRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "Deposit", data);
        return promise.then((data) => QueryGetDepositResponse.decode(new Reader(data)));
    }
    DepositAll(request) {
        const data = QueryAllDepositRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "DepositAll", data);
        return promise.then((data) => QueryAllDepositResponse.decode(new Reader(data)));
    }
    UserDenomDeposit(request) {
        const data = QueryUserDenomDepositRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "UserDenomDeposit", data);
        return promise.then((data) => QueryUserDenomDepositResponse.decode(new Reader(data)));
    }
    Withdraw(request) {
        const data = QueryGetWithdrawRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "Withdraw", data);
        return promise.then((data) => QueryGetWithdrawResponse.decode(new Reader(data)));
    }
    WithdrawAll(request) {
        const data = QueryAllWithdrawRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "WithdrawAll", data);
        return promise.then((data) => QueryAllWithdrawResponse.decode(new Reader(data)));
    }
    FeeData(request) {
        const data = QueryGetFeeDataRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "FeeData", data);
        return promise.then((data) => QueryGetFeeDataResponse.decode(new Reader(data)));
    }
    UserDeposit(request) {
        const data = QueryUserDepositRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "UserDeposit", data);
        return promise.then((data) => QueryUserDepositResponse.decode(new Reader(data)));
    }
    UserDenomLockupDeposit(request) {
        const data = QueryUserDenomLockupDepositRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "UserDenomLockupDeposit", data);
        return promise.then((data) => QueryUserDenomLockupDepositResponse.decode(new Reader(data)));
    }
    UserDenomEpochLockupDeposit(request) {
        const data = QueryUserDenomEpochLockupDepositRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qbank.Query", "UserDenomEpochLockupDeposit", data);
        return promise.then((data) => QueryUserDenomEpochLockupDepositResponse.decode(new Reader(data)));
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
