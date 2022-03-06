// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT MODIFY.
import { SigningStargateClient } from "@cosmjs/stargate";
import { Registry } from "@cosmjs/proto-signing";
import { Api } from "./rest";
import { MsgSendIbcCreatePool } from "./types/intergamm/tx";
import { MsgSendIbcJoinPool } from "./types/intergamm/tx";
import { MsgSendIbcWithdraw } from "./types/intergamm/tx";
import { MsgSendIbcExitPool } from "./types/intergamm/tx";
const types = [
    ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcCreatePool", MsgSendIbcCreatePool],
    ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcJoinPool", MsgSendIbcJoinPool],
    ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcWithdraw", MsgSendIbcWithdraw],
    ["/abag.quasarnode.intergamm.intergamm.MsgSendIbcExitPool", MsgSendIbcExitPool],
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
        msgSendIbcCreatePool: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcCreatePool", value: MsgSendIbcCreatePool.fromPartial(data) }),
        msgSendIbcJoinPool: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcJoinPool", value: MsgSendIbcJoinPool.fromPartial(data) }),
        msgSendIbcWithdraw: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcWithdraw", value: MsgSendIbcWithdraw.fromPartial(data) }),
        msgSendIbcExitPool: (data) => ({ typeUrl: "/abag.quasarnode.intergamm.intergamm.MsgSendIbcExitPool", value: MsgSendIbcExitPool.fromPartial(data) }),
    };
};
const queryClient = async ({ addr: addr } = { addr: "http://localhost:1317" }) => {
    return new Api({ baseUrl: addr });
};
export { txClient, queryClient, };
