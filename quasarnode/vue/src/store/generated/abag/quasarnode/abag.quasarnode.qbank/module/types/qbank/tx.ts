/* eslint-disable */
import { Reader, Writer } from "protobufjs/minimal";

export const protobufPackage = "abag.quasarnode.qbank";

export interface MsgRequestDeposit {
  creator: string;
  riskProfile: string;
  vaultID: string;
  amount: string;
  denom: string;
}

export interface MsgRequestDepositResponse {}

const baseMsgRequestDeposit: object = {
  creator: "",
  riskProfile: "",
  vaultID: "",
  amount: "",
  denom: "",
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
    if (message.amount !== "") {
      writer.uint32(34).string(message.amount);
    }
    if (message.denom !== "") {
      writer.uint32(42).string(message.denom);
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
          message.amount = reader.string();
          break;
        case 5:
          message.denom = reader.string();
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
    if (object.amount !== undefined && object.amount !== null) {
      message.amount = String(object.amount);
    } else {
      message.amount = "";
    }
    if (object.denom !== undefined && object.denom !== null) {
      message.denom = String(object.denom);
    } else {
      message.denom = "";
    }
    return message;
  },

  toJSON(message: MsgRequestDeposit): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.riskProfile !== undefined &&
      (obj.riskProfile = message.riskProfile);
    message.vaultID !== undefined && (obj.vaultID = message.vaultID);
    message.amount !== undefined && (obj.amount = message.amount);
    message.denom !== undefined && (obj.denom = message.denom);
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
    if (object.amount !== undefined && object.amount !== null) {
      message.amount = object.amount;
    } else {
      message.amount = "";
    }
    if (object.denom !== undefined && object.denom !== null) {
      message.denom = object.denom;
    } else {
      message.denom = "";
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

/** Msg defines the Msg service. */
export interface Msg {
  /** this line is used by starport scaffolding # proto/tx/rpc */
  RequestDeposit(
    request: MsgRequestDeposit
  ): Promise<MsgRequestDepositResponse>;
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
