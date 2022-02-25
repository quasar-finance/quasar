// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.

import { StdFee } from "@cosmjs/launchpad";
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry, OfflineSigner, EncodeObject, DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgCreatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgCreatePoolInfo } from "./types/qoracle/tx";
import { MsgCreatePoolPosition } from "./types/qoracle/tx";
import { MsgDeletePoolRanking } from "./types/qoracle/tx";
import { MsgCreatePoolRanking } from "./types/qoracle/tx";
import { MsgUpdatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgUpdatePoolPosition } from "./types/qoracle/tx";
import { MsgUpdatePoolRanking } from "./types/qoracle/tx";
import { MsgDeletePoolSpotPrice } from "./types/qoracle/tx";
import { MsgUpdatePoolInfo } from "./types/qoracle/tx";
import { MsgDeletePoolPosition } from "./types/qoracle/tx";
import { MsgDeletePoolInfo } from "./types/qoracle/tx";


const types = [
  ["/abag.quasarnode.qoracle.MsgCreatePoolSpotPrice", MsgCreatePoolSpotPrice],
  ["/abag.quasarnode.qoracle.MsgCreatePoolInfo", MsgCreatePoolInfo],
  ["/abag.quasarnode.qoracle.MsgCreatePoolPosition", MsgCreatePoolPosition],
  ["/abag.quasarnode.qoracle.MsgDeletePoolRanking", MsgDeletePoolRanking],
  ["/abag.quasarnode.qoracle.MsgCreatePoolRanking", MsgCreatePoolRanking],
  ["/abag.quasarnode.qoracle.MsgUpdatePoolSpotPrice", MsgUpdatePoolSpotPrice],
  ["/abag.quasarnode.qoracle.MsgUpdatePoolPosition", MsgUpdatePoolPosition],
  ["/abag.quasarnode.qoracle.MsgUpdatePoolRanking", MsgUpdatePoolRanking],
  ["/abag.quasarnode.qoracle.MsgDeletePoolSpotPrice", MsgDeletePoolSpotPrice],
  ["/abag.quasarnode.qoracle.MsgUpdatePoolInfo", MsgUpdatePoolInfo],
  ["/abag.quasarnode.qoracle.MsgDeletePoolPosition", MsgDeletePoolPosition],
  ["/abag.quasarnode.qoracle.MsgDeletePoolInfo", MsgDeletePoolInfo],
  
];
export const MissingWalletError = new Error("wallet is required");

export const registry = new Registry(<any>types);

const defaultFee = {
  amount: [],
  gas: "200000",
};

interface TxClientOptions {
  addr: string
}

interface SignAndBroadcastOptions {
  fee: StdFee,
  memo?: string
}

const txClient = async (wallet: OfflineSigner, { addr: addr }: TxClientOptions = { addr: "http://localhost:26657" }) => {
  if (!wallet) throw MissingWalletError;
  let client;
  if (addr) {
    client = await SigningStargateClient.connectWithSigner(addr, wallet, { registry });
  }else{
    client = await SigningStargateClient.offline( wallet, { registry });
  }
  const { address } = (await wallet.getAccounts())[0];

  return {
    signAndBroadcast: (msgs: EncodeObject[], { fee, memo }: SignAndBroadcastOptions = {fee: defaultFee, memo: ""}) => client.signAndBroadcast(address, msgs, fee,memo),
    msgCreatePoolSpotPrice: (data: MsgCreatePoolSpotPrice): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolSpotPrice", value: MsgCreatePoolSpotPrice.fromPartial( data ) }),
    msgCreatePoolInfo: (data: MsgCreatePoolInfo): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolInfo", value: MsgCreatePoolInfo.fromPartial( data ) }),
    msgCreatePoolPosition: (data: MsgCreatePoolPosition): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolPosition", value: MsgCreatePoolPosition.fromPartial( data ) }),
    msgDeletePoolRanking: (data: MsgDeletePoolRanking): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolRanking", value: MsgDeletePoolRanking.fromPartial( data ) }),
    msgCreatePoolRanking: (data: MsgCreatePoolRanking): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolRanking", value: MsgCreatePoolRanking.fromPartial( data ) }),
    msgUpdatePoolSpotPrice: (data: MsgUpdatePoolSpotPrice): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolSpotPrice", value: MsgUpdatePoolSpotPrice.fromPartial( data ) }),
    msgUpdatePoolPosition: (data: MsgUpdatePoolPosition): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolPosition", value: MsgUpdatePoolPosition.fromPartial( data ) }),
    msgUpdatePoolRanking: (data: MsgUpdatePoolRanking): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolRanking", value: MsgUpdatePoolRanking.fromPartial( data ) }),
    msgDeletePoolSpotPrice: (data: MsgDeletePoolSpotPrice): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolSpotPrice", value: MsgDeletePoolSpotPrice.fromPartial( data ) }),
    msgUpdatePoolInfo: (data: MsgUpdatePoolInfo): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolInfo", value: MsgUpdatePoolInfo.fromPartial( data ) }),
    msgDeletePoolPosition: (data: MsgDeletePoolPosition): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolPosition", value: MsgDeletePoolPosition.fromPartial( data ) }),
    msgDeletePoolInfo: (data: MsgDeletePoolInfo): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolInfo", value: MsgDeletePoolInfo.fromPartial( data ) }),
    
  };
};

interface QueryClientOptions {
  addr: string
}

const queryClient = async ({ addr: addr }: QueryClientOptions = { addr: "http://localhost:1317" }) => {
  return new Api({ baseUrl: addr });
};

export {
  txClient,
  queryClient,
};
