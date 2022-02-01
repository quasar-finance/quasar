// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgRequestWithdraw } from "./types/qbank/tx";
import { MsgRequestDeposit } from "./types/qbank/tx";
import { MsgClaimRewards } from "./types/qbank/tx";
const types = [
    ["/abag.quasarnode.qbank.MsgRequestWithdraw", MsgRequestWithdraw],
    ["/abag.quasarnode.qbank.MsgRequestDeposit", MsgRequestDeposit],
    ["/abag.quasarnode.qbank.MsgClaimRewards", MsgClaimRewards],
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
        msgRequestWithdraw: (data) => ({ typeUrl: "/abag.quasarnode.qbank.MsgRequestWithdraw", value: MsgRequestWithdraw.fromPartial(data) }),
        msgRequestDeposit: (data) => ({ typeUrl: "/abag.quasarnode.qbank.MsgRequestDeposit", value: MsgRequestDeposit.fromPartial(data) }),
        msgClaimRewards: (data) => ({ typeUrl: "/abag.quasarnode.qbank.MsgClaimRewards", value: MsgClaimRewards.fromPartial(data) }),
    };
};
const queryClient = async ({ addr: addr } = { addr: "http://localhost:1317" }) => {
    return new Api({ baseUrl: addr });
};
export { txClient, queryClient, };
