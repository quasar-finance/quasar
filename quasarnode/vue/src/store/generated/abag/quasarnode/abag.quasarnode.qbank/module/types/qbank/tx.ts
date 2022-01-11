/* eslint-disable */
import { Reader, Writer } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";

export const protobufPackage = "abag.quasarnode.qbank";

export interface MsgRequestDeposit {
  creator: string;
  riskProfile: string;
  vaultID: string;
  coin: Coin | undefined;
}

export interface MsgRequestDepositResponse {}

export interface MsgRequestWithdraw {
  creator: string;
  riskProfile: string;
  vaultID: string;
  coin: Coin | undefined;
}

export interface MsgRequestWithdrawResponse {}

const baseMsgRequestDeposit: object = {
  creator: "",
  riskProfile: "",
  vaultID: "",
};

export const MsgRequestDeposit = {
  encode(message: MsgRequestDeposit, writer: Writer = Writer.create()): Writer {
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

  decode(input: Reader | Uint8Array, length?: number): MsgRequestDeposit {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgRequestDeposit } as MsgRequestDeposit;
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

  fromJSON(object: any): MsgRequestDeposit {
    const message = { ...baseMsgRequestDeposit } as MsgRequestDeposit;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.riskProfile !== undefined && object.riskProfile !== null) {
      message.riskProfile = String(object.riskProfile);
    } else {
      message.riskProfile = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = String(object.vaultID);
    } else {
      message.vaultID = "";
    }
    if (object.coin !== undefined && object.coin !== null) {
      message.coin = Coin.fromJSON(object.coin);
    } else {
      message.coin = undefined;
    }
    return message;
  },

  toJSON(message: MsgRequestDeposit): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.riskProfile !== undefined &&
      (obj.riskProfile = message.riskProfile);
    message.vaultID !== undefined && (obj.vaultID = message.vaultID);
    message.coin !== undefined &&
      (obj.coin = message.coin ? Coin.toJSON(message.coin) : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgRequestDeposit>): MsgRequestDeposit {
    const message = { ...baseMsgRequestDeposit } as MsgRequestDeposit;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.riskProfile !== undefined && object.riskProfile !== null) {
      message.riskProfile = object.riskProfile;
    } else {
      message.riskProfile = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = object.vaultID;
    } else {
      message.vaultID = "";
    }
    if (object.coin !== undefined && object.coin !== null) {
      message.coin = Coin.fromPartial(object.coin);
    } else {
      message.coin = undefined;
    }
    return message;
  },
};

const baseMsgRequestDepositResponse: object = {};

export const MsgRequestDepositResponse = {
  encode(
    _: MsgRequestDepositResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgRequestDepositResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgRequestDepositResponse,
    } as MsgRequestDepositResponse;
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

  fromJSON(_: any): MsgRequestDepositResponse {
    const message = {
      ...baseMsgRequestDepositResponse,
    } as MsgRequestDepositResponse;
    return message;
  },

  toJSON(_: MsgRequestDepositResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgRequestDepositResponse>
  ): MsgRequestDepositResponse {
    const message = {
      ...baseMsgRequestDepositResponse,
    } as MsgRequestDepositResponse;
    return message;
  },
};

const baseMsgRequestWithdraw: object = {
  creator: "",
  riskProfile: "",
  vaultID: "",
};

export const MsgRequestWithdraw = {
  encode(
    message: MsgRequestWithdraw,
    writer: Writer = Writer.create()
  ): Writer {
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

  decode(input: Reader | Uint8Array, length?: number): MsgRequestWithdraw {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgRequestWithdraw } as MsgRequestWithdraw;
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

  fromJSON(object: any): MsgRequestWithdraw {
    const message = { ...baseMsgRequestWithdraw } as MsgRequestWithdraw;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.riskProfile !== undefined && object.riskProfile !== null) {
      message.riskProfile = String(object.riskProfile);
    } else {
      message.riskProfile = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = String(object.vaultID);
    } else {
      message.vaultID = "";
    }
    if (object.coin !== undefined && object.coin !== null) {
      message.coin = Coin.fromJSON(object.coin);
    } else {
      message.coin = undefined;
    }
    return message;
  },

  toJSON(message: MsgRequestWithdraw): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.riskProfile !== undefined &&
      (obj.riskProfile = message.riskProfile);
    message.vaultID !== undefined && (obj.vaultID = message.vaultID);
    message.coin !== undefined &&
      (obj.coin = message.coin ? Coin.toJSON(message.coin) : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgRequestWithdraw>): MsgRequestWithdraw {
    const message = { ...baseMsgRequestWithdraw } as MsgRequestWithdraw;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.riskProfile !== undefined && object.riskProfile !== null) {
      message.riskProfile = object.riskProfile;
    } else {
      message.riskProfile = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = object.vaultID;
    } else {
      message.vaultID = "";
    }
    if (object.coin !== undefined && object.coin !== null) {
      message.coin = Coin.fromPartial(object.coin);
    } else {
      message.coin = undefined;
    }
    return message;
  },
};

const baseMsgRequestWithdrawResponse: object = {};

export const MsgRequestWithdrawResponse = {
  encode(
    _: MsgRequestWithdrawResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgRequestWithdrawResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgRequestWithdrawResponse,
    } as MsgRequestWithdrawResponse;
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

  fromJSON(_: any): MsgRequestWithdrawResponse {
    const message = {
      ...baseMsgRequestWithdrawResponse,
    } as MsgRequestWithdrawResponse;
    return message;
  },

  toJSON(_: MsgRequestWithdrawResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgRequestWithdrawResponse>
  ): MsgRequestWithdrawResponse {
    const message = {
      ...baseMsgRequestWithdrawResponse,
    } as MsgRequestWithdrawResponse;
    return message;
  },
};

/** Msg defines the Msg service. */
export interface Msg {
  RequestDeposit(
    request: MsgRequestDeposit
  ): Promise<MsgRequestDepositResponse>;
  /** this line is used by starport scaffolding # proto/tx/rpc */
  RequestWithdraw(
    request: MsgRequestWithdraw
  ): Promise<MsgRequestWithdrawResponse>;
}

export class MsgClientImpl implements Msg {
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
  }
  RequestDeposit(
    request: MsgRequestDeposit
  ): Promise<MsgRequestDepositResponse> {
    const data = MsgRequestDeposit.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Msg",
      "RequestDeposit",
      data
    );
    return promise.then((data) =>
      MsgRequestDepositResponse.decode(new Reader(data))
    );
  }

  RequestWithdraw(
    request: MsgRequestWithdraw
  ): Promise<MsgRequestWithdrawResponse> {
    const data = MsgRequestWithdraw.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Msg",
      "RequestWithdraw",
      data
    );
    return promise.then((data) =>
      MsgRequestWithdrawResponse.decode(new Reader(data))
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
