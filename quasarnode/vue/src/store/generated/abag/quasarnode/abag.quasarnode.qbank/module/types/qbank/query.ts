/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { Params } from "../qbank/params";
import { Deposit } from "../qbank/deposit";
import {
  PageRequest,
  PageResponse,
} from "../cosmos/base/query/v1beta1/pagination";

export const protobufPackage = "abag.quasarnode.qbank";

/** QueryParamsRequest is request type for the Query/Params RPC method. */
export interface QueryParamsRequest {}

/** QueryParamsResponse is response type for the Query/Params RPC method. */
export interface QueryParamsResponse {
  /** params holds all the parameters of this module. */
  params: Params | undefined;
}

export interface QueryGetDepositRequest {
  id: number;
}

export interface QueryGetDepositResponse {
  Deposit: Deposit | undefined;
}

export interface QueryAllDepositRequest {
  pagination: PageRequest | undefined;
}

export interface QueryAllDepositResponse {
  Deposit: Deposit[];
  pagination: PageResponse | undefined;
}

export interface QueryUserDenomDepositRequest {
  userAcc: string;
  denom: string;
}

export interface QueryUserDenomDepositResponse {
  amount: number;
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

const baseQueryGetDepositRequest: object = { id: 0 };

export const QueryGetDepositRequest = {
  encode(
    message: QueryGetDepositRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.id !== 0) {
      writer.uint32(8).uint64(message.id);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryGetDepositRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseQueryGetDepositRequest } as QueryGetDepositRequest;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryGetDepositRequest {
    const message = { ...baseQueryGetDepositRequest } as QueryGetDepositRequest;
    if (object.id !== undefined && object.id !== null) {
      message.id = Number(object.id);
    } else {
      message.id = 0;
    }
    return message;
  },

  toJSON(message: QueryGetDepositRequest): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetDepositRequest>
  ): QueryGetDepositRequest {
    const message = { ...baseQueryGetDepositRequest } as QueryGetDepositRequest;
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    } else {
      message.id = 0;
    }
    return message;
  },
};

const baseQueryGetDepositResponse: object = {};

export const QueryGetDepositResponse = {
  encode(
    message: QueryGetDepositResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.Deposit !== undefined) {
      Deposit.encode(message.Deposit, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryGetDepositResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetDepositResponse,
    } as QueryGetDepositResponse;
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

  fromJSON(object: any): QueryGetDepositResponse {
    const message = {
      ...baseQueryGetDepositResponse,
    } as QueryGetDepositResponse;
    if (object.Deposit !== undefined && object.Deposit !== null) {
      message.Deposit = Deposit.fromJSON(object.Deposit);
    } else {
      message.Deposit = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetDepositResponse): unknown {
    const obj: any = {};
    message.Deposit !== undefined &&
      (obj.Deposit = message.Deposit
        ? Deposit.toJSON(message.Deposit)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetDepositResponse>
  ): QueryGetDepositResponse {
    const message = {
      ...baseQueryGetDepositResponse,
    } as QueryGetDepositResponse;
    if (object.Deposit !== undefined && object.Deposit !== null) {
      message.Deposit = Deposit.fromPartial(object.Deposit);
    } else {
      message.Deposit = undefined;
    }
    return message;
  },
};

const baseQueryAllDepositRequest: object = {};

export const QueryAllDepositRequest = {
  encode(
    message: QueryAllDepositRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.pagination !== undefined) {
      PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryAllDepositRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseQueryAllDepositRequest } as QueryAllDepositRequest;
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

  fromJSON(object: any): QueryAllDepositRequest {
    const message = { ...baseQueryAllDepositRequest } as QueryAllDepositRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllDepositRequest): unknown {
    const obj: any = {};
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageRequest.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllDepositRequest>
  ): QueryAllDepositRequest {
    const message = { ...baseQueryAllDepositRequest } as QueryAllDepositRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromPartial(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },
};

const baseQueryAllDepositResponse: object = {};

export const QueryAllDepositResponse = {
  encode(
    message: QueryAllDepositResponse,
    writer: Writer = Writer.create()
  ): Writer {
    for (const v of message.Deposit) {
      Deposit.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.pagination !== undefined) {
      PageResponse.encode(
        message.pagination,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryAllDepositResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllDepositResponse,
    } as QueryAllDepositResponse;
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

  fromJSON(object: any): QueryAllDepositResponse {
    const message = {
      ...baseQueryAllDepositResponse,
    } as QueryAllDepositResponse;
    message.Deposit = [];
    if (object.Deposit !== undefined && object.Deposit !== null) {
      for (const e of object.Deposit) {
        message.Deposit.push(Deposit.fromJSON(e));
      }
    }
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageResponse.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllDepositResponse): unknown {
    const obj: any = {};
    if (message.Deposit) {
      obj.Deposit = message.Deposit.map((e) =>
        e ? Deposit.toJSON(e) : undefined
      );
    } else {
      obj.Deposit = [];
    }
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageResponse.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllDepositResponse>
  ): QueryAllDepositResponse {
    const message = {
      ...baseQueryAllDepositResponse,
    } as QueryAllDepositResponse;
    message.Deposit = [];
    if (object.Deposit !== undefined && object.Deposit !== null) {
      for (const e of object.Deposit) {
        message.Deposit.push(Deposit.fromPartial(e));
      }
    }
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageResponse.fromPartial(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },
};

const baseQueryUserDenomDepositRequest: object = { userAcc: "", denom: "" };

export const QueryUserDenomDepositRequest = {
  encode(
    message: QueryUserDenomDepositRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.userAcc !== "") {
      writer.uint32(10).string(message.userAcc);
    }
    if (message.denom !== "") {
      writer.uint32(18).string(message.denom);
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryUserDenomDepositRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDenomDepositRequest,
    } as QueryUserDenomDepositRequest;
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

  fromJSON(object: any): QueryUserDenomDepositRequest {
    const message = {
      ...baseQueryUserDenomDepositRequest,
    } as QueryUserDenomDepositRequest;
    if (object.userAcc !== undefined && object.userAcc !== null) {
      message.userAcc = String(object.userAcc);
    } else {
      message.userAcc = "";
    }
    if (object.denom !== undefined && object.denom !== null) {
      message.denom = String(object.denom);
    } else {
      message.denom = "";
    }
    return message;
  },

  toJSON(message: QueryUserDenomDepositRequest): unknown {
    const obj: any = {};
    message.userAcc !== undefined && (obj.userAcc = message.userAcc);
    message.denom !== undefined && (obj.denom = message.denom);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDenomDepositRequest>
  ): QueryUserDenomDepositRequest {
    const message = {
      ...baseQueryUserDenomDepositRequest,
    } as QueryUserDenomDepositRequest;
    if (object.userAcc !== undefined && object.userAcc !== null) {
      message.userAcc = object.userAcc;
    } else {
      message.userAcc = "";
    }
    if (object.denom !== undefined && object.denom !== null) {
      message.denom = object.denom;
    } else {
      message.denom = "";
    }
    return message;
  },
};

const baseQueryUserDenomDepositResponse: object = { amount: 0 };

export const QueryUserDenomDepositResponse = {
  encode(
    message: QueryUserDenomDepositResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.amount !== 0) {
      writer.uint32(8).uint64(message.amount);
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryUserDenomDepositResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDenomDepositResponse,
    } as QueryUserDenomDepositResponse;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.amount = longToNumber(reader.uint64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryUserDenomDepositResponse {
    const message = {
      ...baseQueryUserDenomDepositResponse,
    } as QueryUserDenomDepositResponse;
    if (object.amount !== undefined && object.amount !== null) {
      message.amount = Number(object.amount);
    } else {
      message.amount = 0;
    }
    return message;
  },

  toJSON(message: QueryUserDenomDepositResponse): unknown {
    const obj: any = {};
    message.amount !== undefined && (obj.amount = message.amount);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDenomDepositResponse>
  ): QueryUserDenomDepositResponse {
    const message = {
      ...baseQueryUserDenomDepositResponse,
    } as QueryUserDenomDepositResponse;
    if (object.amount !== undefined && object.amount !== null) {
      message.amount = object.amount;
    } else {
      message.amount = 0;
    }
    return message;
  },
};

/** Query defines the gRPC querier service. */
export interface Query {
  /** Parameters queries the parameters of the module. */
  Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
  /** Queries a Deposit by id. */
  Deposit(request: QueryGetDepositRequest): Promise<QueryGetDepositResponse>;
  /** Queries a list of Deposit items. */
  DepositAll(request: QueryAllDepositRequest): Promise<QueryAllDepositResponse>;
  /** Queries a list of UserDenomDeposit items. */
  UserDenomDeposit(
    request: QueryUserDenomDepositRequest
  ): Promise<QueryUserDenomDepositResponse>;
}

export class QueryClientImpl implements Query {
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
  }
  Params(request: QueryParamsRequest): Promise<QueryParamsResponse> {
    const data = QueryParamsRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "Params",
      data
    );
    return promise.then((data) => QueryParamsResponse.decode(new Reader(data)));
  }

  Deposit(request: QueryGetDepositRequest): Promise<QueryGetDepositResponse> {
    const data = QueryGetDepositRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "Deposit",
      data
    );
    return promise.then((data) =>
      QueryGetDepositResponse.decode(new Reader(data))
    );
  }

  DepositAll(
    request: QueryAllDepositRequest
  ): Promise<QueryAllDepositResponse> {
    const data = QueryAllDepositRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "DepositAll",
      data
    );
    return promise.then((data) =>
      QueryAllDepositResponse.decode(new Reader(data))
    );
  }

  UserDenomDeposit(
    request: QueryUserDenomDepositRequest
  ): Promise<QueryUserDenomDepositResponse> {
    const data = QueryUserDenomDepositRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "UserDenomDeposit",
      data
    );
    return promise.then((data) =>
      QueryUserDenomDepositResponse.decode(new Reader(data))
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
