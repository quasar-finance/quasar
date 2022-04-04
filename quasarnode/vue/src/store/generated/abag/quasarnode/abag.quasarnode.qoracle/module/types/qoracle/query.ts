/* eslint-disable */
import { Reader, Writer } from "protobufjs/minimal";
import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";
import {
  PageRequest,
  PageResponse,
} from "../cosmos/base/query/v1beta1/pagination";
import { PoolRanking } from "../qoracle/pool_ranking";
import { PoolSpotPrice } from "../qoracle/pool_spot_price";
import { PoolInfo } from "../qoracle/pool_info";

export const protobufPackage = "abag.quasarnode.qoracle";

/** QueryParamsRequest is request type for the Query/Params RPC method. */
export interface QueryParamsRequest {}

/** QueryParamsResponse is response type for the Query/Params RPC method. */
export interface QueryParamsResponse {
  /** params holds all the parameters of this module. */
  params: Params | undefined;
}

export interface QueryGetPoolPositionRequest {
  poolId: string;
}

export interface QueryGetPoolPositionResponse {
  poolPosition: PoolPosition | undefined;
}

export interface QueryAllPoolPositionRequest {
  pagination: PageRequest | undefined;
}

export interface QueryAllPoolPositionResponse {
  poolPosition: PoolPosition[];
  pagination: PageResponse | undefined;
}

export interface QueryGetPoolRankingRequest {}

export interface QueryGetPoolRankingResponse {
  PoolRanking: PoolRanking | undefined;
}

export interface QueryGetPoolSpotPriceRequest {
  poolId: string;
  denomIn: string;
  denomOut: string;
}

export interface QueryGetPoolSpotPriceResponse {
  poolSpotPrice: PoolSpotPrice | undefined;
}

export interface QueryAllPoolSpotPriceRequest {
  pagination: PageRequest | undefined;
}

export interface QueryAllPoolSpotPriceResponse {
  poolSpotPrice: PoolSpotPrice[];
  pagination: PageResponse | undefined;
}

export interface QueryGetPoolInfoRequest {
  poolId: string;
}

export interface QueryGetPoolInfoResponse {
  poolInfo: PoolInfo | undefined;
}

export interface QueryAllPoolInfoRequest {
  pagination: PageRequest | undefined;
}

export interface QueryAllPoolInfoResponse {
  poolInfo: PoolInfo[];
  pagination: PageResponse | undefined;
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

const baseQueryGetPoolPositionRequest: object = { poolId: "" };

export const QueryGetPoolPositionRequest = {
  encode(
    message: QueryGetPoolPositionRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolId !== "") {
      writer.uint32(10).string(message.poolId);
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
          message.poolId = reader.string();
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
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    return message;
  },

  toJSON(message: QueryGetPoolPositionRequest): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolPositionRequest>
  ): QueryGetPoolPositionRequest {
    const message = {
      ...baseQueryGetPoolPositionRequest,
    } as QueryGetPoolPositionRequest;
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
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
    if (message.poolPosition !== undefined) {
      PoolPosition.encode(
        message.poolPosition,
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
          message.poolPosition = PoolPosition.decode(reader, reader.uint32());
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
    if (object.poolPosition !== undefined && object.poolPosition !== null) {
      message.poolPosition = PoolPosition.fromJSON(object.poolPosition);
    } else {
      message.poolPosition = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetPoolPositionResponse): unknown {
    const obj: any = {};
    message.poolPosition !== undefined &&
      (obj.poolPosition = message.poolPosition
        ? PoolPosition.toJSON(message.poolPosition)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolPositionResponse>
  ): QueryGetPoolPositionResponse {
    const message = {
      ...baseQueryGetPoolPositionResponse,
    } as QueryGetPoolPositionResponse;
    if (object.poolPosition !== undefined && object.poolPosition !== null) {
      message.poolPosition = PoolPosition.fromPartial(object.poolPosition);
    } else {
      message.poolPosition = undefined;
    }
    return message;
  },
};

const baseQueryAllPoolPositionRequest: object = {};

export const QueryAllPoolPositionRequest = {
  encode(
    message: QueryAllPoolPositionRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.pagination !== undefined) {
      PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryAllPoolPositionRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllPoolPositionRequest,
    } as QueryAllPoolPositionRequest;
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

  fromJSON(object: any): QueryAllPoolPositionRequest {
    const message = {
      ...baseQueryAllPoolPositionRequest,
    } as QueryAllPoolPositionRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllPoolPositionRequest): unknown {
    const obj: any = {};
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageRequest.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllPoolPositionRequest>
  ): QueryAllPoolPositionRequest {
    const message = {
      ...baseQueryAllPoolPositionRequest,
    } as QueryAllPoolPositionRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromPartial(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },
};

const baseQueryAllPoolPositionResponse: object = {};

export const QueryAllPoolPositionResponse = {
  encode(
    message: QueryAllPoolPositionResponse,
    writer: Writer = Writer.create()
  ): Writer {
    for (const v of message.poolPosition) {
      PoolPosition.encode(v!, writer.uint32(10).fork()).ldelim();
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
  ): QueryAllPoolPositionResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllPoolPositionResponse,
    } as QueryAllPoolPositionResponse;
    message.poolPosition = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolPosition.push(
            PoolPosition.decode(reader, reader.uint32())
          );
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

  fromJSON(object: any): QueryAllPoolPositionResponse {
    const message = {
      ...baseQueryAllPoolPositionResponse,
    } as QueryAllPoolPositionResponse;
    message.poolPosition = [];
    if (object.poolPosition !== undefined && object.poolPosition !== null) {
      for (const e of object.poolPosition) {
        message.poolPosition.push(PoolPosition.fromJSON(e));
      }
    }
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageResponse.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllPoolPositionResponse): unknown {
    const obj: any = {};
    if (message.poolPosition) {
      obj.poolPosition = message.poolPosition.map((e) =>
        e ? PoolPosition.toJSON(e) : undefined
      );
    } else {
      obj.poolPosition = [];
    }
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageResponse.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllPoolPositionResponse>
  ): QueryAllPoolPositionResponse {
    const message = {
      ...baseQueryAllPoolPositionResponse,
    } as QueryAllPoolPositionResponse;
    message.poolPosition = [];
    if (object.poolPosition !== undefined && object.poolPosition !== null) {
      for (const e of object.poolPosition) {
        message.poolPosition.push(PoolPosition.fromPartial(e));
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

const baseQueryGetPoolRankingRequest: object = {};

export const QueryGetPoolRankingRequest = {
  encode(
    _: QueryGetPoolRankingRequest,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetPoolRankingRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolRankingRequest,
    } as QueryGetPoolRankingRequest;
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

  fromJSON(_: any): QueryGetPoolRankingRequest {
    const message = {
      ...baseQueryGetPoolRankingRequest,
    } as QueryGetPoolRankingRequest;
    return message;
  },

  toJSON(_: QueryGetPoolRankingRequest): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<QueryGetPoolRankingRequest>
  ): QueryGetPoolRankingRequest {
    const message = {
      ...baseQueryGetPoolRankingRequest,
    } as QueryGetPoolRankingRequest;
    return message;
  },
};

const baseQueryGetPoolRankingResponse: object = {};

export const QueryGetPoolRankingResponse = {
  encode(
    message: QueryGetPoolRankingResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.PoolRanking !== undefined) {
      PoolRanking.encode(
        message.PoolRanking,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetPoolRankingResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolRankingResponse,
    } as QueryGetPoolRankingResponse;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.PoolRanking = PoolRanking.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryGetPoolRankingResponse {
    const message = {
      ...baseQueryGetPoolRankingResponse,
    } as QueryGetPoolRankingResponse;
    if (object.PoolRanking !== undefined && object.PoolRanking !== null) {
      message.PoolRanking = PoolRanking.fromJSON(object.PoolRanking);
    } else {
      message.PoolRanking = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetPoolRankingResponse): unknown {
    const obj: any = {};
    message.PoolRanking !== undefined &&
      (obj.PoolRanking = message.PoolRanking
        ? PoolRanking.toJSON(message.PoolRanking)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolRankingResponse>
  ): QueryGetPoolRankingResponse {
    const message = {
      ...baseQueryGetPoolRankingResponse,
    } as QueryGetPoolRankingResponse;
    if (object.PoolRanking !== undefined && object.PoolRanking !== null) {
      message.PoolRanking = PoolRanking.fromPartial(object.PoolRanking);
    } else {
      message.PoolRanking = undefined;
    }
    return message;
  },
};

const baseQueryGetPoolSpotPriceRequest: object = {
  poolId: "",
  denomIn: "",
  denomOut: "",
};

export const QueryGetPoolSpotPriceRequest = {
  encode(
    message: QueryGetPoolSpotPriceRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolId !== "") {
      writer.uint32(10).string(message.poolId);
    }
    if (message.denomIn !== "") {
      writer.uint32(18).string(message.denomIn);
    }
    if (message.denomOut !== "") {
      writer.uint32(26).string(message.denomOut);
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetPoolSpotPriceRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolSpotPriceRequest,
    } as QueryGetPoolSpotPriceRequest;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolId = reader.string();
          break;
        case 2:
          message.denomIn = reader.string();
          break;
        case 3:
          message.denomOut = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryGetPoolSpotPriceRequest {
    const message = {
      ...baseQueryGetPoolSpotPriceRequest,
    } as QueryGetPoolSpotPriceRequest;
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = String(object.denomIn);
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = String(object.denomOut);
    } else {
      message.denomOut = "";
    }
    return message;
  },

  toJSON(message: QueryGetPoolSpotPriceRequest): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    message.denomIn !== undefined && (obj.denomIn = message.denomIn);
    message.denomOut !== undefined && (obj.denomOut = message.denomOut);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolSpotPriceRequest>
  ): QueryGetPoolSpotPriceRequest {
    const message = {
      ...baseQueryGetPoolSpotPriceRequest,
    } as QueryGetPoolSpotPriceRequest;
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    if (object.denomIn !== undefined && object.denomIn !== null) {
      message.denomIn = object.denomIn;
    } else {
      message.denomIn = "";
    }
    if (object.denomOut !== undefined && object.denomOut !== null) {
      message.denomOut = object.denomOut;
    } else {
      message.denomOut = "";
    }
    return message;
  },
};

const baseQueryGetPoolSpotPriceResponse: object = {};

export const QueryGetPoolSpotPriceResponse = {
  encode(
    message: QueryGetPoolSpotPriceResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolSpotPrice !== undefined) {
      PoolSpotPrice.encode(
        message.poolSpotPrice,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetPoolSpotPriceResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolSpotPriceResponse,
    } as QueryGetPoolSpotPriceResponse;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolSpotPrice = PoolSpotPrice.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryGetPoolSpotPriceResponse {
    const message = {
      ...baseQueryGetPoolSpotPriceResponse,
    } as QueryGetPoolSpotPriceResponse;
    if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
      message.poolSpotPrice = PoolSpotPrice.fromJSON(object.poolSpotPrice);
    } else {
      message.poolSpotPrice = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetPoolSpotPriceResponse): unknown {
    const obj: any = {};
    message.poolSpotPrice !== undefined &&
      (obj.poolSpotPrice = message.poolSpotPrice
        ? PoolSpotPrice.toJSON(message.poolSpotPrice)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolSpotPriceResponse>
  ): QueryGetPoolSpotPriceResponse {
    const message = {
      ...baseQueryGetPoolSpotPriceResponse,
    } as QueryGetPoolSpotPriceResponse;
    if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
      message.poolSpotPrice = PoolSpotPrice.fromPartial(object.poolSpotPrice);
    } else {
      message.poolSpotPrice = undefined;
    }
    return message;
  },
};

const baseQueryAllPoolSpotPriceRequest: object = {};

export const QueryAllPoolSpotPriceRequest = {
  encode(
    message: QueryAllPoolSpotPriceRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.pagination !== undefined) {
      PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryAllPoolSpotPriceRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllPoolSpotPriceRequest,
    } as QueryAllPoolSpotPriceRequest;
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

  fromJSON(object: any): QueryAllPoolSpotPriceRequest {
    const message = {
      ...baseQueryAllPoolSpotPriceRequest,
    } as QueryAllPoolSpotPriceRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllPoolSpotPriceRequest): unknown {
    const obj: any = {};
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageRequest.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllPoolSpotPriceRequest>
  ): QueryAllPoolSpotPriceRequest {
    const message = {
      ...baseQueryAllPoolSpotPriceRequest,
    } as QueryAllPoolSpotPriceRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromPartial(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },
};

const baseQueryAllPoolSpotPriceResponse: object = {};

export const QueryAllPoolSpotPriceResponse = {
  encode(
    message: QueryAllPoolSpotPriceResponse,
    writer: Writer = Writer.create()
  ): Writer {
    for (const v of message.poolSpotPrice) {
      PoolSpotPrice.encode(v!, writer.uint32(10).fork()).ldelim();
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
  ): QueryAllPoolSpotPriceResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllPoolSpotPriceResponse,
    } as QueryAllPoolSpotPriceResponse;
    message.poolSpotPrice = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolSpotPrice.push(
            PoolSpotPrice.decode(reader, reader.uint32())
          );
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

  fromJSON(object: any): QueryAllPoolSpotPriceResponse {
    const message = {
      ...baseQueryAllPoolSpotPriceResponse,
    } as QueryAllPoolSpotPriceResponse;
    message.poolSpotPrice = [];
    if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
      for (const e of object.poolSpotPrice) {
        message.poolSpotPrice.push(PoolSpotPrice.fromJSON(e));
      }
    }
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageResponse.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllPoolSpotPriceResponse): unknown {
    const obj: any = {};
    if (message.poolSpotPrice) {
      obj.poolSpotPrice = message.poolSpotPrice.map((e) =>
        e ? PoolSpotPrice.toJSON(e) : undefined
      );
    } else {
      obj.poolSpotPrice = [];
    }
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageResponse.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllPoolSpotPriceResponse>
  ): QueryAllPoolSpotPriceResponse {
    const message = {
      ...baseQueryAllPoolSpotPriceResponse,
    } as QueryAllPoolSpotPriceResponse;
    message.poolSpotPrice = [];
    if (object.poolSpotPrice !== undefined && object.poolSpotPrice !== null) {
      for (const e of object.poolSpotPrice) {
        message.poolSpotPrice.push(PoolSpotPrice.fromPartial(e));
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

const baseQueryGetPoolInfoRequest: object = { poolId: "" };

export const QueryGetPoolInfoRequest = {
  encode(
    message: QueryGetPoolInfoRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolId !== "") {
      writer.uint32(10).string(message.poolId);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryGetPoolInfoRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolInfoRequest,
    } as QueryGetPoolInfoRequest;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryGetPoolInfoRequest {
    const message = {
      ...baseQueryGetPoolInfoRequest,
    } as QueryGetPoolInfoRequest;
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = String(object.poolId);
    } else {
      message.poolId = "";
    }
    return message;
  },

  toJSON(message: QueryGetPoolInfoRequest): unknown {
    const obj: any = {};
    message.poolId !== undefined && (obj.poolId = message.poolId);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolInfoRequest>
  ): QueryGetPoolInfoRequest {
    const message = {
      ...baseQueryGetPoolInfoRequest,
    } as QueryGetPoolInfoRequest;
    if (object.poolId !== undefined && object.poolId !== null) {
      message.poolId = object.poolId;
    } else {
      message.poolId = "";
    }
    return message;
  },
};

const baseQueryGetPoolInfoResponse: object = {};

export const QueryGetPoolInfoResponse = {
  encode(
    message: QueryGetPoolInfoResponse,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.poolInfo !== undefined) {
      PoolInfo.encode(message.poolInfo, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): QueryGetPoolInfoResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryGetPoolInfoResponse,
    } as QueryGetPoolInfoResponse;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolInfo = PoolInfo.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): QueryGetPoolInfoResponse {
    const message = {
      ...baseQueryGetPoolInfoResponse,
    } as QueryGetPoolInfoResponse;
    if (object.poolInfo !== undefined && object.poolInfo !== null) {
      message.poolInfo = PoolInfo.fromJSON(object.poolInfo);
    } else {
      message.poolInfo = undefined;
    }
    return message;
  },

  toJSON(message: QueryGetPoolInfoResponse): unknown {
    const obj: any = {};
    message.poolInfo !== undefined &&
      (obj.poolInfo = message.poolInfo
        ? PoolInfo.toJSON(message.poolInfo)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryGetPoolInfoResponse>
  ): QueryGetPoolInfoResponse {
    const message = {
      ...baseQueryGetPoolInfoResponse,
    } as QueryGetPoolInfoResponse;
    if (object.poolInfo !== undefined && object.poolInfo !== null) {
      message.poolInfo = PoolInfo.fromPartial(object.poolInfo);
    } else {
      message.poolInfo = undefined;
    }
    return message;
  },
};

const baseQueryAllPoolInfoRequest: object = {};

export const QueryAllPoolInfoRequest = {
  encode(
    message: QueryAllPoolInfoRequest,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.pagination !== undefined) {
      PageRequest.encode(message.pagination, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): QueryAllPoolInfoRequest {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllPoolInfoRequest,
    } as QueryAllPoolInfoRequest;
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

  fromJSON(object: any): QueryAllPoolInfoRequest {
    const message = {
      ...baseQueryAllPoolInfoRequest,
    } as QueryAllPoolInfoRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllPoolInfoRequest): unknown {
    const obj: any = {};
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageRequest.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllPoolInfoRequest>
  ): QueryAllPoolInfoRequest {
    const message = {
      ...baseQueryAllPoolInfoRequest,
    } as QueryAllPoolInfoRequest;
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageRequest.fromPartial(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },
};

const baseQueryAllPoolInfoResponse: object = {};

export const QueryAllPoolInfoResponse = {
  encode(
    message: QueryAllPoolInfoResponse,
    writer: Writer = Writer.create()
  ): Writer {
    for (const v of message.poolInfo) {
      PoolInfo.encode(v!, writer.uint32(10).fork()).ldelim();
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
  ): QueryAllPoolInfoResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseQueryAllPoolInfoResponse,
    } as QueryAllPoolInfoResponse;
    message.poolInfo = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.poolInfo.push(PoolInfo.decode(reader, reader.uint32()));
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

  fromJSON(object: any): QueryAllPoolInfoResponse {
    const message = {
      ...baseQueryAllPoolInfoResponse,
    } as QueryAllPoolInfoResponse;
    message.poolInfo = [];
    if (object.poolInfo !== undefined && object.poolInfo !== null) {
      for (const e of object.poolInfo) {
        message.poolInfo.push(PoolInfo.fromJSON(e));
      }
    }
    if (object.pagination !== undefined && object.pagination !== null) {
      message.pagination = PageResponse.fromJSON(object.pagination);
    } else {
      message.pagination = undefined;
    }
    return message;
  },

  toJSON(message: QueryAllPoolInfoResponse): unknown {
    const obj: any = {};
    if (message.poolInfo) {
      obj.poolInfo = message.poolInfo.map((e) =>
        e ? PoolInfo.toJSON(e) : undefined
      );
    } else {
      obj.poolInfo = [];
    }
    message.pagination !== undefined &&
      (obj.pagination = message.pagination
        ? PageResponse.toJSON(message.pagination)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<QueryAllPoolInfoResponse>
  ): QueryAllPoolInfoResponse {
    const message = {
      ...baseQueryAllPoolInfoResponse,
    } as QueryAllPoolInfoResponse;
    message.poolInfo = [];
    if (object.poolInfo !== undefined && object.poolInfo !== null) {
      for (const e of object.poolInfo) {
        message.poolInfo.push(PoolInfo.fromPartial(e));
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

/** Query defines the gRPC querier service. */
export interface Query {
  /** Parameters queries the parameters of the module. */
  Params(request: QueryParamsRequest): Promise<QueryParamsResponse>;
  /** Queries a PoolPosition by index. */
  PoolPosition(
    request: QueryGetPoolPositionRequest
  ): Promise<QueryGetPoolPositionResponse>;
  /** Queries a list of PoolPosition items. */
  PoolPositionAll(
    request: QueryAllPoolPositionRequest
  ): Promise<QueryAllPoolPositionResponse>;
  /** Queries a PoolRanking by index. */
  PoolRanking(
    request: QueryGetPoolRankingRequest
  ): Promise<QueryGetPoolRankingResponse>;
  /** Queries a PoolSpotPrice by index. */
  PoolSpotPrice(
    request: QueryGetPoolSpotPriceRequest
  ): Promise<QueryGetPoolSpotPriceResponse>;
  /** Queries a list of PoolSpotPrice items. */
  PoolSpotPriceAll(
    request: QueryAllPoolSpotPriceRequest
  ): Promise<QueryAllPoolSpotPriceResponse>;
  /** Queries a PoolInfo by index. */
  PoolInfo(request: QueryGetPoolInfoRequest): Promise<QueryGetPoolInfoResponse>;
  /** Queries a list of PoolInfo items. */
  PoolInfoAll(
    request: QueryAllPoolInfoRequest
  ): Promise<QueryAllPoolInfoResponse>;
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

  PoolPositionAll(
    request: QueryAllPoolPositionRequest
  ): Promise<QueryAllPoolPositionResponse> {
    const data = QueryAllPoolPositionRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "PoolPositionAll",
      data
    );
    return promise.then((data) =>
      QueryAllPoolPositionResponse.decode(new Reader(data))
    );
  }

  PoolRanking(
    request: QueryGetPoolRankingRequest
  ): Promise<QueryGetPoolRankingResponse> {
    const data = QueryGetPoolRankingRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "PoolRanking",
      data
    );
    return promise.then((data) =>
      QueryGetPoolRankingResponse.decode(new Reader(data))
    );
  }

  PoolSpotPrice(
    request: QueryGetPoolSpotPriceRequest
  ): Promise<QueryGetPoolSpotPriceResponse> {
    const data = QueryGetPoolSpotPriceRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "PoolSpotPrice",
      data
    );
    return promise.then((data) =>
      QueryGetPoolSpotPriceResponse.decode(new Reader(data))
    );
  }

  PoolSpotPriceAll(
    request: QueryAllPoolSpotPriceRequest
  ): Promise<QueryAllPoolSpotPriceResponse> {
    const data = QueryAllPoolSpotPriceRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "PoolSpotPriceAll",
      data
    );
    return promise.then((data) =>
      QueryAllPoolSpotPriceResponse.decode(new Reader(data))
    );
  }

  PoolInfo(
    request: QueryGetPoolInfoRequest
  ): Promise<QueryGetPoolInfoResponse> {
    const data = QueryGetPoolInfoRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "PoolInfo",
      data
    );
    return promise.then((data) =>
      QueryGetPoolInfoResponse.decode(new Reader(data))
    );
  }

  PoolInfoAll(
    request: QueryAllPoolInfoRequest
  ): Promise<QueryAllPoolInfoResponse> {
    const data = QueryAllPoolInfoRequest.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qoracle.Query",
      "PoolInfoAll",
      data
    );
    return promise.then((data) =>
      QueryAllPoolInfoResponse.decode(new Reader(data))
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
