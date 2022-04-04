/* eslint-disable */
import { Coin } from "../cosmos/base/v1beta1/coin";
import { Writer, Reader } from "protobufjs/minimal";
export const protobufPackage = "abag.quasarnode.qbank";
/** LockupTypes defines different types of locktypes to be used in the system for users deposit */
export var LockupTypes;
(function (LockupTypes) {
    LockupTypes[LockupTypes["Invalid"] = 0] = "Invalid";
    /** Days_7 - 7 Days */
    LockupTypes[LockupTypes["Days_7"] = 1] = "Days_7";
    /** Days_21 - 21 Days of lockup */
    LockupTypes[LockupTypes["Days_21"] = 2] = "Days_21";
    /** Months_1 - 1 Month of lockup */
    LockupTypes[LockupTypes["Months_1"] = 3] = "Months_1";
    /** Months_3 - 3 Months of lockup */
    LockupTypes[LockupTypes["Months_3"] = 4] = "Months_3";
    LockupTypes[LockupTypes["UNRECOGNIZED"] = -1] = "UNRECOGNIZED";
})(LockupTypes || (LockupTypes = {}));
export function lockupTypesFromJSON(object) {
    switch (object) {
        case 0:
        case "Invalid":
            return LockupTypes.Invalid;
        case 1:
        case "Days_7":
            return LockupTypes.Days_7;
        case 2:
        case "Days_21":
            return LockupTypes.Days_21;
        case 3:
        case "Months_1":
            return LockupTypes.Months_1;
        case 4:
        case "Months_3":
            return LockupTypes.Months_3;
        case -1:
        case "UNRECOGNIZED":
        default:
            return LockupTypes.UNRECOGNIZED;
    }
}
export function lockupTypesToJSON(object) {
    switch (object) {
        case LockupTypes.Invalid:
            return "Invalid";
        case LockupTypes.Days_7:
            return "Days_7";
        case LockupTypes.Days_21:
            return "Days_21";
        case LockupTypes.Months_1:
            return "Months_1";
        case LockupTypes.Months_3:
            return "Months_3";
        default:
            return "UNKNOWN";
    }
}
const baseQCoins = {};
export const QCoins = {
    encode(message, writer = Writer.create()) {
        for (const v of message.coins) {
            Coin.encode(v, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseQCoins };
        message.coins = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.coins.push(Coin.decode(reader, reader.uint32()));
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseQCoins };
        message.coins = [];
        if (object.coins !== undefined && object.coins !== null) {
            for (const e of object.coins) {
                message.coins.push(Coin.fromJSON(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        if (message.coins) {
            obj.coins = message.coins.map((e) => (e ? Coin.toJSON(e) : undefined));
        }
        else {
            obj.coins = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseQCoins };
        message.coins = [];
        if (object.coins !== undefined && object.coins !== null) {
            for (const e of object.coins) {
                message.coins.push(Coin.fromPartial(e));
            }
        }
        return message;
    },
};
const baseQDenoms = { denoms: "" };
export const QDenoms = {
    encode(message, writer = Writer.create()) {
        for (const v of message.denoms) {
            writer.uint32(10).string(v);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = { ...baseQDenoms };
        message.denoms = [];
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.denoms.push(reader.string());
                    break;
                default:
                    reader.skipType(tag & 7);
                    break;
            }
        }
        return message;
    },
    fromJSON(object) {
        const message = { ...baseQDenoms };
        message.denoms = [];
        if (object.denoms !== undefined && object.denoms !== null) {
            for (const e of object.denoms) {
                message.denoms.push(String(e));
            }
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        if (message.denoms) {
            obj.denoms = message.denoms.map((e) => e);
        }
        else {
            obj.denoms = [];
        }
        return obj;
    },
    fromPartial(object) {
        const message = { ...baseQDenoms };
        message.denoms = [];
        if (object.denoms !== undefined && object.denoms !== null) {
            for (const e of object.denoms) {
                message.denoms.push(e);
            }
        }
        return message;
    },
};
