import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing'
import {
  alice_mnemnoic,
  bob_mnemnoic,
  charlie_mnemonic,
  quasar_rpc,
  WalletOwners,
} from './config'

export async function getWallet(
  owner: WalletOwners,
): Promise<[SigningCosmWasmClient, DirectSecp256k1HdWallet]> {
  let mnemnoic =
    owner === 'alice'
      ? alice_mnemnoic
      : owner === 'bob'
      ? bob_mnemnoic
      : charlie_mnemonic
  let wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemnoic, {
    prefix: 'quasar',
  })
  let signingClient = await SigningCosmWasmClient.connectWithSigner(
    quasar_rpc,
    wallet,
  )
  return [signingClient, wallet]
}
