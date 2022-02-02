/* eslint-disable */
import { Reader, util, configure, Writer } from "protobufjs/minimal";
import * as Long from "long";
import { Params } from "../qbank/params";
import { Deposit } from "../qbank/deposit";
import {
  PageRequest,
  PageResponse,
} from "../cosmos/base/query/v1beta1/pagination";
import { Withdraw } from "../qbank/withdraw";
import { FeeData } from "../qbank/fee_data";
import { QCoins } from "../qbank/common";

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

export interface QueryGetWithdrawRequest {
  id: number;
}

export interface QueryGetWithdrawResponse {
  Withdraw: Withdraw | undefined;
}

export interface QueryAllWithdrawRequest {
  pagination: PageRequest | undefined;
}

export interface QueryAllWithdrawResponse {
  Withdraw: Withdraw[];
  pagination: PageResponse | undefined;
}

export interface QueryGetFeeDataRequest {}

export interface QueryGetFeeDataResponse {
  FeeData: FeeData | undefined;
}

export interface QueryUserDepositRequest {
  userAcc: string;
}

export interface QueryUserDepositResponse {
  coins: QCoins | undefined;
}

export interface QueryUserDenomLockupDepositRequest {
  userAcc: string;
  denom: string;
  lockupType: string;
}

export interface QueryUserDenomLockupDepositResponse {
  amount: number;
}

export interface QueryUserDenomEpochLockupDepositRequest {
  userAcc: string;
  denom: string;
  epochDay: number[];
  lockupType: string;
}

export interface QueryUserDenomEpochLockupDepositResponse {
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

const baseQueryGetWithdrawRequest: object = { id: 0 };

export const QueryGetWithdrawRequest = {
  encode(
    message: QueryGetWithdrawRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.id !== 0) {
      writer.uint32(8).uint64(message.id);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryGetWithdrawRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetWithdrawRequest,
    } as QueryGetWithdrawRequest;
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

  fromJSON(object: any): QueryGetWithdrawRequest {
    const message = {
      ...baseQueryGetWithdrawRequest,
    } as QueryGetWithdrawRequest;
    if (object.id !== undefined && object.id !== null) {
      message.id = Number(object.id);
    } else {
      message.id = 0;
    }
    return message;
  },

  toJSON(message: QueryGetWithdrawRequest): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetWithdrawRequest>
  ): QueryGetWithdrawRequest {
    const message = {
      ...baseQueryGetWithdrawRequest,
    } as QueryGetWithdrawRequest;
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    } else {
      message.id = 0;
    }
    return message;
  },
};

const baseQueryGetWithdrawResponse: object = {};

export const QueryGetWithdrawResponse = {
  encode(
    message: QueryGetWithdrawResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.Withdraw !== undefined) {
      Withdraw.encode(message.Withdraw, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetWithdrawResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetWithdrawResponse,
    } as QueryGetWithdrawResponse;
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

  fromJSON(object: any): QueryGetWithdrawResponse {
    const message = {
      ...baseQueryGetWithdrawResponse,
    } as QueryGetWithdrawResponse;
    if (object.Withdraw !== undefined && object.Withdraw !== null) {
      message.Withdraw = Withdraw.fromJSON(object.Withdraw);
    } else {
      message.Withdraw = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetWithdrawResponse): unknown {
    const obj: any = {};
    message.Withdraw !== undefined &&
      (obj.Withdraw = message.Withdraw
        ? Withdraw.toJSON(message.Withdraw)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetWithdrawResponse>
  ): QueryGetWithdrawResponse {
    const message = {
      ...baseQueryGetWithdrawResponse,
    } as QueryGetWithdrawResponse;
    if (object.Withdraw !== undefined && object.Withdraw !== null) {
      message.Withdraw = Withdraw.fromPartial(object.Withdraw);
    } else {
      message.Withdraw = undefined;
    }
    return message;
  },
};

const baseQueryAllWithdrawRequest: object = {};

export const QueryAllWithdrawRequest = {
  encode(
    message: QueryAllWithdrawRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.pagination !== undefined) {
      PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryAllWithdrawRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllWithdrawRequest,
    } as QueryAllWithdrawRequest;
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

  fromJSON(object: any): QueryAllWithdrawRequest {
    const message = {
      ...baseQueryAllWithdrawRequest,
    } as QueryAllWithdrawRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllWithdrawRequest): unknown {
    const obj: any = {};
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageRequest.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllWithdrawRequest>
  ): QueryAllWithdrawRequest {
    const message = {
      ...baseQueryAllWithdrawRequest,
    } as QueryAllWithdrawRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromPartial(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },
};

const baseQueryAllWithdrawResponse: object = {};

export const QueryAllWithdrawResponse = {
  encode(
    message: QueryAllWithdrawResponse,
    writer: Writer = Writer.create()
  ): Writer {
    for (const v of message.Withdraw) {
      Withdraw.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.pagination !== undefined) {
      PageResponse.encode(
        message.pagination,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryAllWithdrawResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllWithdrawResponse,
    } as QueryAllWithdrawResponse;
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

  fromJSON(object: any): QueryAllWithdrawResponse {
    const message = {
      ...baseQueryAllWithdrawResponse,
    } as QueryAllWithdrawResponse;
    message.Withdraw = [];
    if (object.Withdraw !== undefined && object.Withdraw !== null) {
      for (const e of object.Withdraw) {
        message.Withdraw.push(Withdraw.fromJSON(e));
      }
    }
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageResponse.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllWithdrawResponse): unknown {
    const obj: any = {};
    if (message.Withdraw) {
      obj.Withdraw = message.Withdraw.map((e) =>
        e ? Withdraw.toJSON(e) : undefined
      );
    } else {
      obj.Withdraw = [];
    }
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageResponse.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllWithdrawResponse>
  ): QueryAllWithdrawResponse {
    const message = {
      ...baseQueryAllWithdrawResponse,
    } as QueryAllWithdrawResponse;
    message.Withdraw = [];
    if (object.Withdraw !== undefined && object.Withdraw !== null) {
      for (const e of object.Withdraw) {
        message.Withdraw.push(Withdraw.fromPartial(e));
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

const baseQueryGetFeeDataRequest: object = {};

export const QueryGetFeeDataRequest = {
  encode(_: QueryGetFeeDataRequest, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryGetFeeDataRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseQueryGetFeeDataRequest } as QueryGetFeeDataRequest;
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

  fromJSON(_: any): QueryGetFeeDataRequest {
    const message = { ...baseQueryGetFeeDataRequest } as QueryGetFeeDataRequest;
    return message;
  },

  toJSON(_: QueryGetFeeDataRequest): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(_: DeepPartial<QueryGetFeeDataRequest>): QueryGetFeeDataRequest {
    const message = { ...baseQueryGetFeeDataRequest } as QueryGetFeeDataRequest;
    return message;
  },
};

const baseQueryGetFeeDataResponse: object = {};

export const QueryGetFeeDataResponse = {
  encode(
    message: QueryGetFeeDataResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.FeeData !== undefined) {
      FeeData.encode(message.FeeData, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryGetFeeDataResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetFeeDataResponse,
    } as QueryGetFeeDataResponse;
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

  fromJSON(object: any): QueryGetFeeDataResponse {
    const message = {
      ...baseQueryGetFeeDataResponse,
    } as QueryGetFeeDataResponse;
    if (object.FeeData !== undefined && object.FeeData !== null) {
      message.FeeData = FeeData.fromJSON(object.FeeData);
    } else {
      message.FeeData = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetFeeDataResponse): unknown {
    const obj: any = {};
    message.FeeData !== undefined &&
      (obj.FeeData = message.FeeData
        ? FeeData.toJSON(message.FeeData)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetFeeDataResponse>
  ): QueryGetFeeDataResponse {
    const message = {
      ...baseQueryGetFeeDataResponse,
    } as QueryGetFeeDataResponse;
    if (object.FeeData !== undefined && object.FeeData !== null) {
      message.FeeData = FeeData.fromPartial(object.FeeData);
    } else {
      message.FeeData = undefined;
    }
    return message;
  },
};

const baseQueryUserDepositRequest: object = { userAcc: "" };

export const QueryUserDepositRequest = {
  encode(
    message: QueryUserDepositRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.userAcc !== "") {
      writer.uint32(10).string(message.userAcc);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryUserDepositRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDepositRequest,
    } as QueryUserDepositRequest;
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

  fromJSON(object: any): QueryUserDepositRequest {
    const message = {
      ...baseQueryUserDepositRequest,
    } as QueryUserDepositRequest;
    if (object.userAcc !== undefined && object.userAcc !== null) {
      message.userAcc = String(object.userAcc);
    } else {
      message.userAcc = "";
    }
    return message;
  },

  toJSON(message: QueryUserDepositRequest): unknown {
    const obj: any = {};
    message.userAcc !== undefined && (obj.userAcc = message.userAcc);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDepositRequest>
  ): QueryUserDepositRequest {
    const message = {
      ...baseQueryUserDepositRequest,
    } as QueryUserDepositRequest;
    if (object.userAcc !== undefined && object.userAcc !== null) {
      message.userAcc = object.userAcc;
    } else {
      message.userAcc = "";
    }
    return message;
  },
};

const baseQueryUserDepositResponse: object = {};

export const QueryUserDepositResponse = {
  encode(
    message: QueryUserDepositResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.coins !== undefined) {
      QCoins.encode(message.coins, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryUserDepositResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDepositResponse,
    } as QueryUserDepositResponse;
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

  fromJSON(object: any): QueryUserDepositResponse {
    const message = {
      ...baseQueryUserDepositResponse,
    } as QueryUserDepositResponse;
    if (object.coins !== undefined && object.coins !== null) {
      message.coins = QCoins.fromJSON(object.coins);
    } else {
      message.coins = undefined;
    }
    return message;
  },

  toJSON(message: QueryUserDepositResponse): unknown {
    const obj: any = {};
    message.coins !== undefined &&
      (obj.coins = message.coins ? QCoins.toJSON(message.coins) : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDepositResponse>
  ): QueryUserDepositResponse {
    const message = {
      ...baseQueryUserDepositResponse,
    } as QueryUserDepositResponse;
    if (object.coins !== undefined && object.coins !== null) {
      message.coins = QCoins.fromPartial(object.coins);
    } else {
      message.coins = undefined;
    }
    return message;
  },
};

const baseQueryUserDenomLockupDepositRequest: object = {
  userAcc: "",
  denom: "",
  lockupType: "",
};

export const QueryUserDenomLockupDepositRequest = {
  encode(
    message: QueryUserDenomLockupDepositRequest,
    writer: Writer = Writer.create()
  ): Writer {
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

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryUserDenomLockupDepositRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDenomLockupDepositRequest,
    } as QueryUserDenomLockupDepositRequest;
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

  fromJSON(object: any): QueryUserDenomLockupDepositRequest {
    const message = {
      ...baseQueryUserDenomLockupDepositRequest,
    } as QueryUserDenomLockupDepositRequest;
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
    if (object.lockupType !== undefined && object.lockupType !== null) {
      message.lockupType = String(object.lockupType);
    } else {
      message.lockupType = "";
    }
    return message;
  },

  toJSON(message: QueryUserDenomLockupDepositRequest): unknown {
    const obj: any = {};
    message.userAcc !== undefined && (obj.userAcc = message.userAcc);
    message.denom !== undefined && (obj.denom = message.denom);
    message.lockupType !== undefined && (obj.lockupType = message.lockupType);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDenomLockupDepositRequest>
  ): QueryUserDenomLockupDepositRequest {
    const message = {
      ...baseQueryUserDenomLockupDepositRequest,
    } as QueryUserDenomLockupDepositRequest;
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
    if (object.lockupType !== undefined && object.lockupType !== null) {
      message.lockupType = object.lockupType;
    } else {
      message.lockupType = "";
    }
    return message;
  },
};

const baseQueryUserDenomLockupDepositResponse: object = { amount: 0 };

export const QueryUserDenomLockupDepositResponse = {
  encode(
    message: QueryUserDenomLockupDepositResponse,
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
  ): QueryUserDenomLockupDepositResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDenomLockupDepositResponse,
    } as QueryUserDenomLockupDepositResponse;
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

  fromJSON(object: any): QueryUserDenomLockupDepositResponse {
    const message = {
      ...baseQueryUserDenomLockupDepositResponse,
    } as QueryUserDenomLockupDepositResponse;
    if (object.amount !== undefined && object.amount !== null) {
      message.amount = Number(object.amount);
    } else {
      message.amount = 0;
    }
    return message;
  },

  toJSON(message: QueryUserDenomLockupDepositResponse): unknown {
    const obj: any = {};
    message.amount !== undefined && (obj.amount = message.amount);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDenomLockupDepositResponse>
  ): QueryUserDenomLockupDepositResponse {
    const message = {
      ...baseQueryUserDenomLockupDepositResponse,
    } as QueryUserDenomLockupDepositResponse;
    if (object.amount !== undefined && object.amount !== null) {
      message.amount = object.amount;
    } else {
      message.amount = 0;
    }
    return message;
  },
};

const baseQueryUserDenomEpochLockupDepositRequest: object = {
  userAcc: "",
  denom: "",
  epochDay: 0,
  lockupType: "",
};

export const QueryUserDenomEpochLockupDepositRequest = {
  encode(
    message: QueryUserDenomEpochLockupDepositRequest,
    writer: Writer = Writer.create()
  ): Writer {
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

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryUserDenomEpochLockupDepositRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDenomEpochLockupDepositRequest,
    } as QueryUserDenomEpochLockupDepositRequest;
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
              message.epochDay.push(longToNumber(reader.uint64() as Long));
            }
          } else {
            message.epochDay.push(longToNumber(reader.uint64() as Long));
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

  fromJSON(object: any): QueryUserDenomEpochLockupDepositRequest {
    const message = {
      ...baseQueryUserDenomEpochLockupDepositRequest,
    } as QueryUserDenomEpochLockupDepositRequest;
    message.epochDay = [];
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
    if (object.epochDay !== undefined && object.epochDay !== null) {
      for (const e of object.epochDay) {
        message.epochDay.push(Number(e));
      }
    }
    if (object.lockupType !== undefined && object.lockupType !== null) {
      message.lockupType = String(object.lockupType);
    } else {
      message.lockupType = "";
    }
    return message;
  },

  toJSON(message: QueryUserDenomEpochLockupDepositRequest): unknown {
    const obj: any = {};
    message.userAcc !== undefined && (obj.userAcc = message.userAcc);
    message.denom !== undefined && (obj.denom = message.denom);
    if (message.epochDay) {
      obj.epochDay = message.epochDay.map((e) => e);
    } else {
      obj.epochDay = [];
    }
    message.lockupType !== undefined && (obj.lockupType = message.lockupType);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDenomEpochLockupDepositRequest>
  ): QueryUserDenomEpochLockupDepositRequest {
    const message = {
      ...baseQueryUserDenomEpochLockupDepositRequest,
    } as QueryUserDenomEpochLockupDepositRequest;
    message.epochDay = [];
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
    if (object.epochDay !== undefined && object.epochDay !== null) {
      for (const e of object.epochDay) {
        message.epochDay.push(e);
      }
    }
    if (object.lockupType !== undefined && object.lockupType !== null) {
      message.lockupType = object.lockupType;
    } else {
      message.lockupType = "";
    }
    return message;
  },
};

const baseQueryUserDenomEpochLockupDepositResponse: object = { amount: 0 };

export const QueryUserDenomEpochLockupDepositResponse = {
  encode(
    message: QueryUserDenomEpochLockupDepositResponse,
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
  ): QueryUserDenomEpochLockupDepositResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryUserDenomEpochLockupDepositResponse,
    } as QueryUserDenomEpochLockupDepositResponse;
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

  fromJSON(object: any): QueryUserDenomEpochLockupDepositResponse {
    const message = {
      ...baseQueryUserDenomEpochLockupDepositResponse,
    } as QueryUserDenomEpochLockupDepositResponse;
    if (object.amount !== undefined && object.amount !== null) {
      message.amount = Number(object.amount);
    } else {
      message.amount = 0;
    }
    return message;
  },

  toJSON(message: QueryUserDenomEpochLockupDepositResponse): unknown {
    const obj: any = {};
    message.amount !== undefined && (obj.amount = message.amount);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryUserDenomEpochLockupDepositResponse>
  ): QueryUserDenomEpochLockupDepositResponse {
    const message = {
      ...baseQueryUserDenomEpochLockupDepositResponse,
    } as QueryUserDenomEpochLockupDepositResponse;
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
  /** Queries a Withdraw by id. */
  Withdraw(request: QueryGetWithdrawRequest): Promise<QueryGetWithdrawResponse>;
  /** Queries a list of Withdraw items. */
  WithdrawAll(
    request: QueryAllWithdrawRequest
  ): Promise<QueryAllWithdrawResponse>;
  /** Queries a FeeData by index. */
  FeeData(request: QueryGetFeeDataRequest): Promise<QueryGetFeeDataResponse>;
  /** Queries a list of UserDeposit items. */
  UserDeposit(
    request: QueryUserDepositRequest
  ): Promise<QueryUserDepositResponse>;
  /** Queries a list of UserDenomLockupDeposit items. */
  UserDenomLockupDeposit(
    request: QueryUserDenomLockupDepositRequest
  ): Promise<QueryUserDenomLockupDepositResponse>;
  /** Queries a list of UserDenomEpochLockupDeposit items. */
  UserDenomEpochLockupDeposit(
    request: QueryUserDenomEpochLockupDepositRequest
  ): Promise<QueryUserDenomEpochLockupDepositResponse>;
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

  Withdraw(
    request: QueryGetWithdrawRequest
  ): Promise<QueryGetWithdrawResponse> {
    const data = QueryGetWithdrawRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "Withdraw",
      data
    );
    return promise.then((data) =>
      QueryGetWithdrawResponse.decode(new Reader(data))
    );
  }

  WithdrawAll(
    request: QueryAllWithdrawRequest
  ): Promise<QueryAllWithdrawResponse> {
    const data = QueryAllWithdrawRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "WithdrawAll",
      data
    );
    return promise.then((data) =>
      QueryAllWithdrawResponse.decode(new Reader(data))
    );
  }

  FeeData(request: QueryGetFeeDataRequest): Promise<QueryGetFeeDataResponse> {
    const data = QueryGetFeeDataRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "FeeData",
      data
    );
    return promise.then((data) =>
      QueryGetFeeDataResponse.decode(new Reader(data))
    );
  }

  UserDeposit(
    request: QueryUserDepositRequest
  ): Promise<QueryUserDepositResponse> {
    const data = QueryUserDepositRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "UserDeposit",
      data
    );
    return promise.then((data) =>
      QueryUserDepositResponse.decode(new Reader(data))
    );
  }

  UserDenomLockupDeposit(
    request: QueryUserDenomLockupDepositRequest
  ): Promise<QueryUserDenomLockupDepositResponse> {
    const data = QueryUserDenomLockupDepositRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "UserDenomLockupDeposit",
      data
    );
    return promise.then((data) =>
      QueryUserDenomLockupDepositResponse.decode(new Reader(data))
    );
  }

  UserDenomEpochLockupDeposit(
    request: QueryUserDenomEpochLockupDepositRequest
  ): Promise<QueryUserDenomEpochLockupDepositResponse> {
    const data = QueryUserDenomEpochLockupDepositRequest.encode(
      request
    ).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Query",
      "UserDenomEpochLockupDeposit",
      data
    );
    return promise.then((data) =>
      QueryUserDenomEpochLockupDepositResponse.decode(new Reader(data))
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
