import { StdFee } from '@cosmjs/amino'
import { BasicVaultClient } from '../BasicVault.client'
import { Coin } from '../BasicVault.types'
import { FEE_DENOM, OSMO_DENOM, WalletOwners } from './config'
import { getWallet } from './wallet'

let stdFee: StdFee = {
  amount: [
    {
      denom: FEE_DENOM,
      amount: '100',
    },
  ],
  gas: '1300000',
}

async function getVault(from: WalletOwners, vaultAddress: string) {
  let [signingClient, wallet] = await getWallet(from)
  let address = (await wallet.getAccounts())[0].address

  let basicVaultClient = new BasicVaultClient(
    signingClient,
    address,
    vaultAddress,
  )
  return basicVaultClient
}

export async function bond({
  from,
  vaultAddress,
  funds,
}: {
  from: WalletOwners
  vaultAddress: string
  funds: Coin[]
}) {
  console.log('Bonding ' + from + '...')

  let basicVaultClient = await getVault(from, vaultAddress)
  return basicVaultClient.bond({}, stdFee, 'memo teehee', funds)
}

export async function start_unbond({
  from,
  vaultAddress,
  amount,
}: {
  from: WalletOwners
  vaultAddress: string
  amount: string //uint128
}) {
  await new Promise((r) => setTimeout(r, 5000))
  console.log('Start unbond ' + from + '...')

  let basicVaultClient = await getVault(from, vaultAddress)
  return basicVaultClient.unbond({ amount }, stdFee, 'memo teehee', [])
}

export async function claim({
  from,
  vaultAddress,
}: {
  from: WalletOwners
  vaultAddress: string
}) {
  console.log('Unbonding ' + from + '...')

  let basicVaultClient = await getVault(from, vaultAddress)
  return basicVaultClient.claim(stdFee, 'memo teehee', [])
}

export async function try_icq({
  vaultAddress,
  from,
}: {
  vaultAddress: string
  from: WalletOwners
}) {
  console.log('Trying ICQ manually...')

  let basicVaultClient = await getVault(from, vaultAddress)
  return basicVaultClient.clearCache(stdFee, 'memo teehee', [])
}

// query
export async function getBalance(vaultAddress: string, of: WalletOwners) {
  let [_, wallet] = await getWallet(of)
  let basicVaultClient = await getVault('alice', vaultAddress)
  const address = (await wallet.getAccounts())[0].address

  let balances = await basicVaultClient.balance({ address })

  return balances
}

export async function getPendingBonds(vaultAddress: string, of: WalletOwners) {
  let [_, wallet] = await getWallet(of)
  let basicVaultClient = await getVault('alice', vaultAddress)
  const address = (await wallet.getAccounts())[0].address

  let pendingBonds = await basicVaultClient.pendingBonds({ address })
  return pendingBonds
}

export async function getPendingUnbonds(
  vaultAddress: string,
  of: WalletOwners,
) {
  let [_, wallet] = await getWallet(of)
  let basicVaultClient = await getVault('alice', vaultAddress)
  const address = (await wallet.getAccounts())[0].address

  let pendingUnbonds = await basicVaultClient.pendingUnbonds({ address })
  return pendingUnbonds
}

// chain
export async function getChainBalance(of: WalletOwners) {
  let [client, wallet] = await getWallet(of)
  const address = (await wallet.getAccounts())[0].address

  const balance = await client.getBalance(address, OSMO_DENOM)
  return balance
}
