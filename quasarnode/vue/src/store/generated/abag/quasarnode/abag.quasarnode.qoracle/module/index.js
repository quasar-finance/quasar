// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgDeletePoolRanking } from "./types/qoracle/tx";
import { MsgDeletePoolSpotPrice } from "./types/qoracle/tx";
import { MsgUpdatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgCreatePoolSpotPrice } from "./types/qoracle/tx";
import { MsgCreatePoolRanking } from "./types/qoracle/tx";
import { MsgDeletePoolPosition } from "./types/qoracle/tx";
import { MsgDeletePoolInfo } from "./types/qoracle/tx";
import { MsgCreatePoolPosition } from "./types/qoracle/tx";
import { MsgUpdatePoolPosition } from "./types/qoracle/tx";
import { MsgUpdatePoolRanking } from "./types/qoracle/tx";
import { MsgCreatePoolInfo } from "./types/qoracle/tx";
import { MsgUpdatePoolInfo } from "./types/qoracle/tx";
const types = [
    ["/abag.quasarnode.qoracle.MsgDeletePoolRanking", MsgDeletePoolRanking],
    ["/abag.quasarnode.qoracle.MsgDeletePoolSpotPrice", MsgDeletePoolSpotPrice],
    ["/abag.quasarnode.qoracle.MsgUpdatePoolSpotPrice", MsgUpdatePoolSpotPrice],
    ["/abag.quasarnode.qoracle.MsgCreatePoolSpotPrice", MsgCreatePoolSpotPrice],
    ["/abag.quasarnode.qoracle.MsgCreatePoolRanking", MsgCreatePoolRanking],
    ["/abag.quasarnode.qoracle.MsgDeletePoolPosition", MsgDeletePoolPosition],
    ["/abag.quasarnode.qoracle.MsgDeletePoolInfo", MsgDeletePoolInfo],
    ["/abag.quasarnode.qoracle.MsgCreatePoolPosition", MsgCreatePoolPosition],
    ["/abag.quasarnode.qoracle.MsgUpdatePoolPosition", MsgUpdatePoolPosition],
    ["/abag.quasarnode.qoracle.MsgUpdatePoolRanking", MsgUpdatePoolRanking],
    ["/abag.quasarnode.qoracle.MsgCreatePoolInfo", MsgCreatePoolInfo],
    ["/abag.quasarnode.qoracle.MsgUpdatePoolInfo", MsgUpdatePoolInfo],
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
        msgDeletePoolRanking: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolRanking", value: MsgDeletePoolRanking.fromPartial(data) }),
        msgDeletePoolSpotPrice: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolSpotPrice", value: MsgDeletePoolSpotPrice.fromPartial(data) }),
        msgUpdatePoolSpotPrice: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolSpotPrice", value: MsgUpdatePoolSpotPrice.fromPartial(data) }),
        msgCreatePoolSpotPrice: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolSpotPrice", value: MsgCreatePoolSpotPrice.fromPartial(data) }),
        msgCreatePoolRanking: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolRanking", value: MsgCreatePoolRanking.fromPartial(data) }),
        msgDeletePoolPosition: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolPosition", value: MsgDeletePoolPosition.fromPartial(data) }),
        msgDeletePoolInfo: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgDeletePoolInfo", value: MsgDeletePoolInfo.fromPartial(data) }),
        msgCreatePoolPosition: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolPosition", value: MsgCreatePoolPosition.fromPartial(data) }),
        msgUpdatePoolPosition: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolPosition", value: MsgUpdatePoolPosition.fromPartial(data) }),
        msgUpdatePoolRanking: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolRanking", value: MsgUpdatePoolRanking.fromPartial(data) }),
        msgCreatePoolInfo: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgCreatePoolInfo", value: MsgCreatePoolInfo.fromPartial(data) }),
        msgUpdatePoolInfo: (data) => ({ typeUrl: "/abag.quasarnode.qoracle.MsgUpdatePoolInfo", value: MsgUpdatePoolInfo.fromPartial(data) }),
    };
};
const queryClient = async ({ addr: addr } = { addr: "http://localhost:1317" }) => {
    return new Api({ baseUrl: addr });
};
export { txClient, queryClient, };
