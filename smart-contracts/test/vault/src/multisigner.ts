import { EncodeObject, coins, encodePubkey } from "@cosmjs/proto-signing";
import { getVault } from "./vault";
import { getWallet } from "./wallet";
import {
  SignerData,
  SigningStargateClient,
  StargateClient,
} from "@cosmjs/stargate";
import { MsgSend } from "cosmjs-types/cosmos/bank/v1beta1/tx";
import {
  MultisigThresholdPubkey,
  Secp256k1HdWallet,
  makeCosmoshubPath,
  StdFee,
  encodeSecp256k1Pubkey,
  createMultisigThresholdPubkey,
  pubkeyToAddress,
} from "@cosmjs/amino";
import { alice_mnemnoic } from "./config";
import { TxRaw } from "cosmjs-types/cosmos/tx/v1beta1/tx";
import { fromBech32 } from "@cosmjs/encoding";
import { AuthInfo, SignerInfo } from "cosmjs-types/cosmos/tx/v1beta1/tx";
import {
  CompactBitArray,
  MultiSignature,
} from "cosmjs-types/cosmos/crypto/multisig/v1beta1/multisig";
import { SignMode } from "cosmjs-types/cosmos/tx/signing/v1beta1/signing";
import Long from "long";

export interface MsgSendEncodeObject extends EncodeObject {
  readonly typeUrl: "/cosmos.bank.v1beta1.MsgSend";
  readonly value: Partial<MsgSend>;
}

export async function multisign() {
  const multisigAccountAddress =
    "cosmos1h90ml36rcu7yegwduzgzderj2jmq49hcpfclw9";

  // On the composer's machine signing instructions are created.
  // The composer does not need to be one of the signers.
  const signingInstruction = await (async () => {
    const client = await StargateClient.connect("temp-url");
    const accountOnChain = await client.getAccount(multisigAccountAddress);

    if (!accountOnChain) throw new Error("Account dont exist on chain");

    const msgSend: MsgSend = {
      fromAddress: multisigAccountAddress,
      toAddress: "cosmos19rvl6ja9h0erq9dc2xxfdzypc739ej8k5esnhg",
      amount: coins(1234, "ucosm"),
    };
    const msg: MsgSendEncodeObject = {
      typeUrl: "/cosmos.bank.v1beta1.MsgSend",
      value: msgSend,
    };
    const gasLimit = 200000;
    const fee = {
      amount: coins(2000, "ucosm"),
      gas: gasLimit.toString(),
    };

    return {
      accountNumber: accountOnChain.accountNumber,
      sequence: accountOnChain.sequence,
      chainId: await client.getChainId(),
      msgs: [msg],
      fee: fee,
      memo: "Use your tokens wisely",
    };
  })();

  const [
    [pubkey0, signature0, bodyBytes],
    [pubkey1, signature1],
    [pubkey2, signature2],
    [pubkey3, signature3],
    [pubkey4, signature4],
  ] = await Promise.all(
    [0, 1, 2, 3, 4].map(async (i) => {
      // Signing environment
      const wallet = await Secp256k1HdWallet.fromMnemonic(alice_mnemnoic, {
        hdPaths: [makeCosmoshubPath(i)],
      });
      const pubkey = encodeSecp256k1Pubkey(
        (await wallet.getAccounts())[0].pubkey
      );
      const address = (await wallet.getAccounts())[0].address;
      const signingClient = await SigningStargateClient.offline(wallet);
      const signerData: SignerData = {
        accountNumber: signingInstruction.accountNumber,
        sequence: signingInstruction.sequence,
        chainId: signingInstruction.chainId,
      };
      const { bodyBytes: bb, signatures } = await signingClient.sign(
        address,
        signingInstruction.msgs,
        signingInstruction.fee,
        signingInstruction.memo,
        signerData
      );
      return [pubkey, signatures[0], bb] as const;
    })
  );

  // From here on, no private keys are required anymore. Any anonymous entity
  // can collect, assemble and broadcast.
  {
    const multisigPubkey = createMultisigThresholdPubkey(
      [pubkey0, pubkey1, pubkey2, pubkey3, pubkey4],
      2
    );
    console.log(
      pubkeyToAddress(multisigPubkey, "cosmos"),
      multisigAccountAddress
    );

    const address0 = pubkeyToAddress(pubkey0, "cosmos");
    const address1 = pubkeyToAddress(pubkey1, "cosmos");
    const address2 = pubkeyToAddress(pubkey2, "cosmos");
    const address3 = pubkeyToAddress(pubkey3, "cosmos");
    const address4 = pubkeyToAddress(pubkey4, "cosmos");

    const broadcaster = await StargateClient.connect("temp-url");
    const signedTx = makeMultisignedTxBytes(
      multisigPubkey,
      signingInstruction.sequence,
      signingInstruction.fee,
      bodyBytes,
      new Map<string, Uint8Array>([
        [address0, signature0],
        [address1, signature1],
        [address2, signature2],
        [address3, signature3],
        [address4, signature4],
      ])
    );
    // ensure signature is valid
    const result = await broadcaster.broadcastTx(signedTx);
    console.log(result);
  }
}

export function makeCompactBitArray(bits: readonly boolean[]): CompactBitArray {
  const byteCount = Math.ceil(bits.length / 8);
  const extraBits = bits.length - Math.floor(bits.length / 8) * 8;
  const bytes = new Uint8Array(byteCount); // zero-filled

  bits.forEach((value, index) => {
    const bytePos = Math.floor(index / 8);
    const bitPos = index % 8;
    // eslint-disable-next-line no-bitwise
    if (value) bytes[bytePos] |= 0b1 << (8 - 1 - bitPos);
  });

  return CompactBitArray.fromPartial({
    elems: bytes,
    extraBitsStored: extraBits,
  });
}

/**
 * Creates a signed transaction from signer info, transaction body and signatures.
 * The result can be broadcasted after serialization.
 *
 * Consider using `makeMultisignedTxBytes` instead if you want to broadcast the
 * transaction immediately.
 */
export function makeMultisignedTx(
  multisigPubkey: MultisigThresholdPubkey,
  sequence: number,
  fee: StdFee,
  bodyBytes: Uint8Array,
  signatures: Map<string, Uint8Array>
): TxRaw {
  const addresses = Array.from(signatures.keys());
  const prefix = fromBech32(addresses[0]).prefix;

  const signers: boolean[] = Array(multisigPubkey.value.pubkeys.length).fill(
    false
  );
  const signaturesList = new Array<Uint8Array>();
  for (let i = 0; i < multisigPubkey.value.pubkeys.length; i++) {
    const signerAddress = pubkeyToAddress(
      multisigPubkey.value.pubkeys[i],
      prefix
    );
    const signature = signatures.get(signerAddress);
    if (signature) {
      signers[i] = true;
      signaturesList.push(signature);
    }
  }

  const signerInfo: SignerInfo = {
    publicKey: encodePubkey(multisigPubkey),
    modeInfo: {
      multi: {
        bitarray: makeCompactBitArray(signers),
        modeInfos: signaturesList.map((_) => ({
          single: { mode: SignMode.SIGN_MODE_LEGACY_AMINO_JSON },
        })),
      },
    },
    sequence: Long.fromNumber(sequence),
  };

  const authInfo = AuthInfo.fromPartial({
    signerInfos: [signerInfo],
    fee: {
      amount: [...fee.amount],
      gasLimit: Long.fromString(fee.gas),
    },
  });

  const authInfoBytes = AuthInfo.encode(authInfo).finish();
  const signedTx = TxRaw.fromPartial({
    bodyBytes: bodyBytes,
    authInfoBytes: authInfoBytes,
    signatures: [
      MultiSignature.encode(
        MultiSignature.fromPartial({ signatures: signaturesList })
      ).finish(),
    ],
  });
  return signedTx;
}

/**
 * Creates a signed transaction from signer info, transaction body and signatures.
 * The result can be broadcasted.
 *
 * This is a wrapper around `makeMultisignedTx` that encodes the transaction for broadcasting.
 */
export function makeMultisignedTxBytes(
  multisigPubkey: MultisigThresholdPubkey,
  sequence: number,
  fee: StdFee,
  bodyBytes: Uint8Array,
  signatures: Map<string, Uint8Array>
): Uint8Array {
  const signedTx = makeMultisignedTx(
    multisigPubkey,
    sequence,
    fee,
    bodyBytes,
    signatures
  );
  return Uint8Array.from(TxRaw.encode(signedTx).finish());
}
