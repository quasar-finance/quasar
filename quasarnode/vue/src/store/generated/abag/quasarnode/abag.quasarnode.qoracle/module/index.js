// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgUpdatePoolPosition } from "./types/qoracle/tx";
import { MsgBalancerPool } from "./types/qoracle/tx";
import { MsgCreatePoolPosition } from "./types/qoracle/tx";
import { MsgDeletePoolPosition } from "./types/qoracle/tx";
const types = [
    ["/abag.quasarnode.qoracle.MsgUpdatePoolPosition", MsgUpdatePoolPosition],
    ["/abag.quasarnode.qoracle.MsgBalancerPool", MsgBalancerPool],
    ["/abag.quasarnode.qoracle.MsgCreatePoolPosition", MsgCreatePoolPosition],
    ["/abag.quasarnode.qoracle.MsgDeletePoolPosition", MsgDeletePoolPosition],
];
export const MissingWalletError = new Error("wallet is required");
export const registry = new Registry(types);
const defaultFee = {
    amount: [],
    gas: "200000",
};
const txClient = async (wallet, { addr: addr } = { addr: "http://localhost:26657" }) => {
    if (!wallet)
        throw MissingWalletError;
    let client;
    if (addr) {
        client = await SigningStargateClient.connectWithSigner(addr, wallet, { registry });
    }
    else {
        client = await SigningStargateClient.offline(wallet, { registry });
    }
    const { address } = (await wallet.getAccounts())[0];
    return {
        signAndBroadcast: (msgs, { fee, memo } = { fee: defaultFee, memo: "" }) => client.signAndBroadcast(address, msgs, fee, memo),
        msgUpdatePoolPosition: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolPosition", value: MsgUpdatePoolPosition.fromPartial(data) }),
        msgBalancerPool: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgBalancerPool", value: MsgBalancerPool.fromPartial(data) }),
        msgCreatePoolPosition: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolPosition", value: MsgCreatePoolPosition.fromPartial(data) }),
        msgDeletePoolPosition: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolPosition", value: MsgDeletePoolPosition.fromPartial(data) }),
    };
};
const queryClient = async ({ addr: addr } = { addr: "http://localhost:1317" }) => {
    return new Api({ baseUrl: addr });
};
export { txClient, queryClient, };
