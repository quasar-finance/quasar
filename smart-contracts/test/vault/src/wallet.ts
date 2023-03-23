import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing'
import { alice_mnemnoic, bob_mnemnoic, quasar_rpc } from './config'

export async function getWallet(
  owner: 'alice' | 'bob',
): Promise<[SigningCosmWasmClient, DirectSecp256k1HdWallet]> {
  let mnemnoic = owner === 'alice' ? alice_mnemnoic : bob_mnemnoic
  let wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemnoic, {
    prefix: 'quasar',
  })
  let signingClient = await SigningCosmWasmClient.connectWithSigner(
    quasar_rpc,
    wallet,
  )
  return [signingClient, wallet]
}
