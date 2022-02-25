// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.

import { StdFee } from "@cosmjs/launchpad";
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry, OfflineSigner, EncodeObject, DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgSendIbcCreatePool } from "./types/intergamm/tx";
import { MsgSendIbcExitPool } from "./types/intergamm/tx";
import { MsgSendIbcJoinPool } from "./types/intergamm/tx";
import { MsgSendIbcWithdraw } from "./types/intergamm/tx";


const types = [
  ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcCreatePool", MsgSendIbcCreatePool],
  ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcExitPool", MsgSendIbcExitPool],
  ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcJoinPool", MsgSendIbcJoinPool],
  ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcWithdraw", MsgSendIbcWithdraw],
  
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
    msgSendIbcCreatePool: (data: MsgSendIbcCreatePool): EncodeObject => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcCreatePool", value: MsgSendIbcCreatePool.fromPartial( data ) }),
    msgSendIbcExitPool: (data: MsgSendIbcExitPool): EncodeObject => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcExitPool", value: MsgSendIbcExitPool.fromPartial( data ) }),
    msgSendIbcJoinPool: (data: MsgSendIbcJoinPool): EncodeObject => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcJoinPool", value: MsgSendIbcJoinPool.fromPartial( data ) }),
    msgSendIbcWithdraw: (data: MsgSendIbcWithdraw): EncodeObject => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcWithdraw", value: MsgSendIbcWithdraw.fromPartial( data ) }),
    
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
