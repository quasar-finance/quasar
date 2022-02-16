// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.

import { StdFee } from "@cosmjs/launchpad";
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry, OfflineSigner, EncodeObject, DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgRequestDeposit } from "./types/qbank/tx";
import { MsgRequestWithdraw } from "./types/qbank/tx";
import { MsgRequestWithdrawAll } from "./types/qbank/tx";
import { MsgClaimRewards } from "./types/qbank/tx";


const types = [
  ["/abag.quasarnode.qbank.MsgRequestDeposit", MsgRequestDeposit],
  ["/abag.quasarnode.qbank.MsgRequestWithdraw", MsgRequestWithdraw],
  ["/abag.quasarnode.qbank.MsgRequestWithdrawAll", MsgRequestWithdrawAll],
  ["/abag.quasarnode.qbank.MsgClaimRewards", MsgClaimRewards],
  
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
    msgRequestDeposit: (data: MsgRequestDeposit): EncodeObject => ({ typeUrl: "/abag.quasarnode.qbank.MsgRequestDeposit", value: MsgRequestDeposit.fromPartial( data ) }),
    msgRequestWithdraw: (data: MsgRequestWithdraw): EncodeObject => ({ typeUrl: "/abag.quasarnode.qbank.MsgRequestWithdraw", value: MsgRequestWithdraw.fromPartial( data ) }),
    msgRequestWithdrawAll: (data: MsgRequestWithdrawAll): EncodeObject => ({ typeUrl: "/abag.quasarnode.qbank.MsgRequestWithdrawAll", value: MsgRequestWithdrawAll.fromPartial( data ) }),
    msgClaimRewards: (data: MsgClaimRewards): EncodeObject => ({ typeUrl: "/abag.quasarnode.qbank.MsgClaimRewards", value: MsgClaimRewards.fromPartial( data ) }),
    
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
