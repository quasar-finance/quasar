/* eslint-disable */
import {
  LockupTypes,
  lockupTypesFromJSON,
  lockupTypesToJSON,
} from "../qbank/common";
import { Reader, Writer } from "protobufjs/minimal";
import { Coin } from "../cosmos/base/v1beta1/coin";

export const protobufPackage = "abag.quasarnode.qbank";

export interface MsgRequestDeposit {
  creator: string;
  riskProfile: string;
  vaultID: string;
  coin: Coin | undefined;
  /** string lockupPeriod = 5; */
  lockupPeriod: LockupTypes;
}

export interface MsgRequestDepositResponse {}

export interface MsgRequestWithdraw {
  creator: string;
  riskProfile: string;
  vaultID: string;
  coin: Coin | undefined;
}

export interface MsgRequestWithdrawResponse {}

/**
 * MsgClaimRewards is tx message to claim all available rewards from the input vault.
 * TODO - Should it move to a separate reward module to avoid cyclic dependencies between modules.
 */
export interface MsgClaimRewards {
  creator: string;
  vaultID: string;
}

export interface MsgClaimRewardsResponse {}

/** MsgRequestWithdrawAll is tx message to withdraw all withdrawable amount from input vault. */
export interface MsgRequestWithdrawAll {
  creator: string;
  vaultID: string;
}

export interface MsgRequestWithdrawAllResponse {}

const baseMsgRequestDeposit: object = {
  creator: "",
  riskProfile: "",
  vaultID: "",
  lockupPeriod: 0,
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
    if (message.lockupPeriod !== 0) {
      writer.uint32(40).int32(message.lockupPeriod);
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
        case 5:
          message.lockupPeriod = reader.int32() as any;
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
    if (object.lockupPeriod !== undefined && object.lockupPeriod !== null) {
      message.lockupPeriod = lockupTypesFromJSON(object.lockupPeriod);
    } else {
      message.lockupPeriod = 0;
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
    message.lockupPeriod !== undefined &&
      (obj.lockupPeriod = lockupTypesToJSON(message.lockupPeriod));
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
    if (object.lockupPeriod !== undefined && object.lockupPeriod !== null) {
      message.lockupPeriod = object.lockupPeriod;
    } else {
      message.lockupPeriod = 0;
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

const baseMsgClaimRewards: object = { creator: "", vaultID: "" };

export const MsgClaimRewards = {
  encode(message: MsgClaimRewards, writer: Writer = Writer.create()): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.vaultID !== "") {
      writer.uint32(18).string(message.vaultID);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgClaimRewards {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgClaimRewards } as MsgClaimRewards;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.vaultID = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgClaimRewards {
    const message = { ...baseMsgClaimRewards } as MsgClaimRewards;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = String(object.vaultID);
    } else {
      message.vaultID = "";
    }
    return message;
  },

  toJSON(message: MsgClaimRewards): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.vaultID !== undefined && (obj.vaultID = message.vaultID);
    return obj;
  },

  fromPartial(object: DeepPartial<MsgClaimRewards>): MsgClaimRewards {
    const message = { ...baseMsgClaimRewards } as MsgClaimRewards;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = object.vaultID;
    } else {
      message.vaultID = "";
    }
    return message;
  },
};

const baseMsgClaimRewardsResponse: object = {};

export const MsgClaimRewardsResponse = {
  encode(_: MsgClaimRewardsResponse, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgClaimRewardsResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgClaimRewardsResponse,
    } as MsgClaimRewardsResponse;
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

  fromJSON(_: any): MsgClaimRewardsResponse {
    const message = {
      ...baseMsgClaimRewardsResponse,
    } as MsgClaimRewardsResponse;
    return message;
  },

  toJSON(_: MsgClaimRewardsResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgClaimRewardsResponse>
  ): MsgClaimRewardsResponse {
    const message = {
      ...baseMsgClaimRewardsResponse,
    } as MsgClaimRewardsResponse;
    return message;
  },
};

const baseMsgRequestWithdrawAll: object = { creator: "", vaultID: "" };

export const MsgRequestWithdrawAll = {
  encode(
    message: MsgRequestWithdrawAll,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.creator !== "") {
      writer.uint32(10).string(message.creator);
    }
    if (message.vaultID !== "") {
      writer.uint32(18).string(message.vaultID);
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): MsgRequestWithdrawAll {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseMsgRequestWithdrawAll } as MsgRequestWithdrawAll;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.creator = reader.string();
          break;
        case 2:
          message.vaultID = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MsgRequestWithdrawAll {
    const message = { ...baseMsgRequestWithdrawAll } as MsgRequestWithdrawAll;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = String(object.creator);
    } else {
      message.creator = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = String(object.vaultID);
    } else {
      message.vaultID = "";
    }
    return message;
  },

  toJSON(message: MsgRequestWithdrawAll): unknown {
    const obj: any = {};
    message.creator !== undefined && (obj.creator = message.creator);
    message.vaultID !== undefined && (obj.vaultID = message.vaultID);
    return obj;
  },

  fromPartial(
    object: DeepPartial<MsgRequestWithdrawAll>
  ): MsgRequestWithdrawAll {
    const message = { ...baseMsgRequestWithdrawAll } as MsgRequestWithdrawAll;
    if (object.creator !== undefined && object.creator !== null) {
      message.creator = object.creator;
    } else {
      message.creator = "";
    }
    if (object.vaultID !== undefined && object.vaultID !== null) {
      message.vaultID = object.vaultID;
    } else {
      message.vaultID = "";
    }
    return message;
  },
};

const baseMsgRequestWithdrawAllResponse: object = {};

export const MsgRequestWithdrawAllResponse = {
  encode(
    _: MsgRequestWithdrawAllResponse,
    writer: Writer = Writer.create()
  ): Writer {
    return writer;
  },

  decode(
    input: Reader | Uint8Array,
    length?: number
  ): MsgRequestWithdrawAllResponse {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = {
      ...baseMsgRequestWithdrawAllResponse,
    } as MsgRequestWithdrawAllResponse;
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

  fromJSON(_: any): MsgRequestWithdrawAllResponse {
    const message = {
      ...baseMsgRequestWithdrawAllResponse,
    } as MsgRequestWithdrawAllResponse;
    return message;
  },

  toJSON(_: MsgRequestWithdrawAllResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial(
    _: DeepPartial<MsgRequestWithdrawAllResponse>
  ): MsgRequestWithdrawAllResponse {
    const message = {
      ...baseMsgRequestWithdrawAllResponse,
    } as MsgRequestWithdrawAllResponse;
    return message;
  },
};

/** Msg defines the Msg service. */
export interface Msg {
  RequestDeposit(
    request: MsgRequestDeposit
  ): Promise<MsgRequestDepositResponse>;
  RequestWithdraw(
    request: MsgRequestWithdraw
  ): Promise<MsgRequestWithdrawResponse>;
  ClaimRewards(request: MsgClaimRewards): Promise<MsgClaimRewardsResponse>;
  /** this line is used by starport scaffolding # proto/tx/rpc */
  RequestWithdrawAll(
    request: MsgRequestWithdrawAll
  ): Promise<MsgRequestWithdrawAllResponse>;
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

  ClaimRewards(request: MsgClaimRewards): Promise<MsgClaimRewardsResponse> {
    const data = MsgClaimRewards.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Msg",
      "ClaimRewards",
      data
    );
    return promise.then((data) =>
      MsgClaimRewardsResponse.decode(new Reader(data))
    );
  }

  RequestWithdrawAll(
    request: MsgRequestWithdrawAll
  ): Promise<MsgRequestWithdrawAllResponse> {
    const data = MsgRequestWithdrawAll.encode(request).finish();
    const promise = this.rpc.request(
      "abag.quasarnode.qbank.Msg",
      "RequestWithdrawAll",
      data
    );
    return promise.then((data) =>
      MsgRequestWithdrawAllResponse.decode(new Reader(data))
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
