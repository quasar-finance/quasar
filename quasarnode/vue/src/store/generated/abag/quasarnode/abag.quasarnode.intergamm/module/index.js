// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgJoinPool } from "./types/intergamm/tx";
import { MsgRegisterAccount } from "./types/intergamm/tx";
import { MsgIbcTransfer } from "./types/intergamm/tx";
import { MsgCreatePool } from "./types/intergamm/tx";
import { MsgExitPool } from "./types/intergamm/tx";
const types = [
    ["/abag.quasarnode.intergamm.MsgJoinPool", MsgJoinPool],
    ["/abag.quasarnode.intergamm.MsgRegisterAccount", MsgRegisterAccount],
    ["/abag.quasarnode.intergamm.MsgIbcTransfer", MsgIbcTransfer],
    ["/abag.quasarnode.intergamm.MsgCreatePool", MsgCreatePool],
    ["/abag.quasarnode.intergamm.MsgExitPool", MsgExitPool],
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
        msgJoinPool: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.MsgJoinPool", value: MsgJoinPool.fromPartial(data) }),
        msgRegisterAccount: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.MsgRegisterAccount", value: MsgRegisterAccount.fromPartial(data) }),
        msgIbcTransfer: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.MsgIbcTransfer", value: MsgIbcTransfer.fromPartial(data) }),
        msgCreatePool: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.MsgCreatePool", value: MsgCreatePool.fromPartial(data) }),
        msgExitPool: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.MsgExitPool", value: MsgExitPool.fromPartial(data) }),
    };
};
const queryClient = async ({ addr: addr } = { addr: "http://localhost:1317" }) => {
    return new Api({ baseUrl: addr });
};
export { txClient, queryClient, };
