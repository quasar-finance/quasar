// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.

import { StdFee } from "@cosmjs/launchpad";
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry, OfflineSigner, EncodeObject, DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgUpdatePoolPosition } from "./types/qoracle/tx";
import { MsgDeletePoolPosition } from "./types/qoracle/tx";
import { MsgCreatePoolPosition } from "./types/qoracle/tx";


const types = [
  ["/abag.quasarnode.qoracle.MsgUpdatePoolPosition", MsgUpdatePoolPosition],
  ["/abag.quasarnode.qoracle.MsgDeletePoolPosition", MsgDeletePoolPosition],
  ["/abag.quasarnode.qoracle.MsgCreatePoolPosition", MsgCreatePoolPosition],
  
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
    msgUpdatePoolPosition: (data: MsgUpdatePoolPosition): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolPosition", value: MsgUpdatePoolPosition.fromPartial( data ) }),
    msgDeletePoolPosition: (data: MsgDeletePoolPosition): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolPosition", value: MsgDeletePoolPosition.fromPartial( data ) }),
    msgCreatePoolPosition: (data: MsgCreatePoolPosition): EncodeObject => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolPosition", value: MsgCreatePoolPosition.fromPartial( data ) }),
    
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
