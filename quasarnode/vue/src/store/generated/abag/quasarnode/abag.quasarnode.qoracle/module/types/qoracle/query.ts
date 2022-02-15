/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";

export const protobufPackage = "abag.quasarnode.qoracle";

/** QueryParamsRequest is request type for the Query/Params RPC method. */
export interface QueryParamsRequest {}

/** QueryParamsResponse is response type for the Query/Params RPC method. */
export interface QueryParamsResponse {
  /** params holds all the parameters of this module. */
  params: Params | undefined;
}

export interface QueryGetPoolPositionRequest {
  PoolID: number;
}

export interface QueryGetPoolPositionResponse {
  PoolPosition: PoolPosition | undefined;
}

const baseQueryParamsRequest: object = {};

export const QueryParamsRequest = {
  encode(_: QueryParamsRequest, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryParamsRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseQueryParamsRequest } as QueryParamsRequest;
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

  fromJSON(_: any): QueryParamsRequest {
    const message = { ...baseQueryParamsRequest } as QueryParamsRequest;
    return message;
  },

  toJSON(_: QueryParamsRequest): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<QueryParamsRequest>): QueryParamsRequest {
    const message = { ...baseQueryParamsRequest } as QueryParamsRequest;
    return message;
  },
};

const baseQueryParamsResponse: object = {};

export const QueryParamsResponse = {
  encode(
    message: QueryParamsResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.params !== undefined) {
      Params.encode(message.params, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryParamsResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseQueryParamsResponse } as QueryParamsResponse;
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

  fromJSON(object: any): QueryParamsResponse {
    const message = { ...baseQueryParamsResponse } as QueryParamsResponse;
    if (object.params !== undefined && object.params !== null) {
      message.params = Params.fromJSON(object.params);
    } else {
      message.params = undefined;
    }
    return message;
  },

  toJSON(message: QueryParamsResponse): unknown {
    const obj: any = {};
    message.params !== undefined &&
      (obj.params = message.params ? Params.toJSON(message.params) : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<QueryParamsResponse>): QueryParamsResponse {
    const message = { ...baseQueryParamsResponse } as QueryParamsResponse;
    if (object.params !== undefined && object.params !== null) {
      message.params = Params.fromPartial(object.params);
    } else {
      message.params = undefined;
    }
    return message;
  },
};

const baseQueryGetPoolPositionRequest: object = { PoolID: 0 };

export const QueryGetPoolPositionRequest = {
  encode(
    message: QueryGetPoolPositionRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.PoolID !== 0) {
      writer.uint32(8).uint64(message.PoolID);
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetPoolPositionRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolPositionRequest,
    } as QueryGetPoolPositionRequest;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.PoolID = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryGetPoolPositionRequest {
    const message = {
      ...baseQueryGetPoolPositionRequest,
    } as QueryGetPoolPositionRequest;
    if (object.PoolID !== undefined && object.PoolID !== null) {
      message.PoolID = Number(object.PoolID);
    } else {
      message.PoolID = 0;
    }
    return message;
  },

  toJSON(message: QueryGetPoolPositionRequest): unknown {
    const obj: any = {};
    message.PoolID !== undefined && (obj.PoolID = message.PoolID);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolPositionRequest>
  ): QueryGetPoolPositionRequest {
    const message = {
      ...baseQueryGetPoolPositionRequest,
    } as QueryGetPoolPositionRequest;
    if (object.PoolID !== undefined && object.PoolID !== null) {
      message.PoolID = object.PoolID;
    } else {
      message.PoolID = 0;
    }
    return message;
  },
};

const baseQueryGetPoolPositionResponse: object = {};

export const QueryGetPoolPositionResponse = {
  encode(
    message: QueryGetPoolPositionResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.PoolPosition !== undefined) {
      PoolPosition.encode(
        message.PoolPosition,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetPoolPositionResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolPositionResponse,
    } as QueryGetPoolPositionResponse;
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

  fromJSON(object: any): QueryGetPoolPositionResponse {
    const message = {
      ...baseQueryGetPoolPositionResponse,
    } as QueryGetPoolPositionResponse;
    if (object.PoolPosition !== undefined && object.PoolPosition !== null) {
      message.PoolPosition = PoolPosition.fromJSON(object.PoolPosition);
    } else {
      message.PoolPosition = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetPoolPositionResponse): unknown {
    const obj: any = {};
    message.PoolPosition !== undefined &&
      (obj.PoolPosition = message.PoolPosition
        ? PoolPosition.toJSON(message.PoolPosition)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolPositionResponse>
  ): QueryGetPoolPositionResponse {
    const message = {
      ...baseQueryGetPoolPositionResponse,
    } as QueryGetPoolPositionResponse;
    if (object.PoolPosition !== undefined && object.PoolPosition !== null) {
      message.PoolPosition = PoolPosition.fromPartial(object.PoolPosition);
    } else {
      message.PoolPosition = undefined;
    }
    return message;
  },
};

/** Query defines the gRPC querier service. */
export interface Query {
  /** Parameters queries the parameters of the module. */
  Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
  /** Queries a PoolPosition by index. */
  PoolPosition(
    request: QueryGetPoolPositionRequest
  ): Promise<QueryGetPoolPositionResponse>;
}

export class QueryClientImpl implements Query {
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
  }
  Params(request: QueryParamsRequest): Promise<QueryParamsResponse> {
    const data = QueryParamsRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "Params",
      data
    );
    return promise.then((data) => QueryParamsResponse.decode(new Reader(data)));
  }

  PoolPosition(
    request: QueryGetPoolPositionRequest
  ): Promise<QueryGetPoolPositionResponse> {
    const data = QueryGetPoolPositionRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "PoolPosition",
      data
    );
    return promise.then((data) =>
      QueryGetPoolPositionResponse.decode(new Reader(data))
    );
  }
}

interface Rpc {
  request(
    service: string,
    method: string,
    data: Uint8Array
  ): Promise<Uint8Array>;
}

declare var self: any | undefined;
declare var window: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") return globalThis;
  if (typeof self !== "undefined") return self;
  if (typeof window !== "undefined") return window;
  if (typeof global !== "undefined") return global;
  throw "Unable to locate global object";
})();

type Builtin = Date | Function | Uint8Array | string | number | undefined;
export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

if (util.Long !== Long) {
  util.Long = Long as any;
  configure();
}
