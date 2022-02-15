/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";
export const protobufPackage = "abag.quasarnode.qoracle";
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
const baseQueryGetPoolPositionRequest = { PoolID: 0 };
export const QueryGetPoolPositionRequest = {
    encode(message, writer = Writer.create()) {
        if (message.PoolID !== 0) {
            writer.uint32(8).uint64(message.PoolID);
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolPositionRequest,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.PoolID = longToNumber(reader.uint64());
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
            ...baseQueryGetPoolPositionRequest,
        };
        if (object.PoolID !== undefined && object.PoolID !== null) {
            message.PoolID = Number(object.PoolID);
        }
        else {
            message.PoolID = 0;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.PoolID !== undefined && (obj.PoolID = message.PoolID);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolPositionRequest,
        };
        if (object.PoolID !== undefined && object.PoolID !== null) {
            message.PoolID = object.PoolID;
        }
        else {
            message.PoolID = 0;
        }
        return message;
    },
};
const baseQueryGetPoolPositionResponse = {};
export const QueryGetPoolPositionResponse = {
    encode(message, writer = Writer.create()) {
        if (message.PoolPosition !== undefined) {
            PoolPosition.encode(message.PoolPosition, writer.uint32(10).fork()).ldelim();
        }
        return writer;
    },
    decode(input, length) {
        const reader = input instanceof Uint8Array ? new Reader(input) : input;
        let end = length === undefined ? reader.len : reader.pos + length;
        const message = {
            ...baseQueryGetPoolPositionResponse,
        };
        while (reader.pos < end) {
            const tag = reader.uint32();
            switch (tag >>> 3) {
                case 1:
                    message.PoolPosition = PoolPosition.decode(reader, reader.uint32());
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
            ...baseQueryGetPoolPositionResponse,
        };
        if (object.PoolPosition !== undefined && object.PoolPosition !== null) {
            message.PoolPosition = PoolPosition.fromJSON(object.PoolPosition);
        }
        else {
            message.PoolPosition = undefined;
        }
        return message;
    },
    toJSON(message) {
        const obj = {};
        message.PoolPosition !== undefined &&
            (obj.PoolPosition = message.PoolPosition
                ? PoolPosition.toJSON(message.PoolPosition)
                : undefined);
        return obj;
    },
    fromPartial(object) {
        const message = {
            ...baseQueryGetPoolPositionResponse,
        };
        if (object.PoolPosition !== undefined && object.PoolPosition !== null) {
            message.PoolPosition = PoolPosition.fromPartial(object.PoolPosition);
        }
        else {
            message.PoolPosition = undefined;
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
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "Params", data);
        return promise.then((data) => QueryParamsResponse.decode(new Reader(data)));
    }
    PoolPosition(request) {
        const data = QueryGetPoolPositionRequest.encode(request).finish();
        const promise = this.rpc.request("abag.quasarnode.qoracle.Query", "PoolPosition", data);
        return promise.then((data) => QueryGetPoolPositionResponse.decode(new Reader(data)));
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
