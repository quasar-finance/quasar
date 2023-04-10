import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { Buffer } from 'buffer'
import { LpStrategyQueryClient } from '../contracts/LpStrategy.client'
import { VAULT_ADDRESS } from '../utils/config'

import { BasicVaultQueryClient } from '../contracts/BasicVault.client'
import { PrimitiveConfig } from '../contracts/BasicVault.types'
import { InvestmentInfo } from '../contracts/BasicVault.types'

let instance: Querier | null = null
let afterInitializationActions = [] as any[]

export default class Querier {
  rpcUrl: string
  chainId: string
  queryClient: CosmWasmClient | undefined
  constructor(RPC_URL: string, CHAIN_ID: string) {
    this.rpcUrl = RPC_URL
    this.chainId = CHAIN_ID
  }

  static async getInstance(rpcUrl: string, chainId: string): Promise<Querier> {
    if (!instance) {
      instance = new Querier(rpcUrl, chainId)
      await instance.init()
    }
    return instance
  }

  async init() {
    this.queryClient = await CosmWasmClient.connect(this.rpcUrl)
    for (let action of afterInitializationActions) {
      action()
    }
  }

  static onInit(action: any) {
    afterInitializationActions.push(action)
  }

  async getAllCodes() {
    return this.queryClient!.getCodes()
  }

  async getCodeDetails(code_id: number) {
    return this.queryClient!.getCodeDetails(code_id)
  }

  async getContractsFromCodeId(code_id: number) {
    return this.queryClient!.getContracts(code_id)
  }

  async getMetadataFromContract(address: string, code_id: number) {
    let contract = await this.queryClient!.getContract(address)
    let contractHistory = await this.queryClient!.getContractCodeHistory(
      address,
    )
    let code = await this.queryClient!.getCodeDetails(code_id)
    let contractName = await this.queryClient!.queryContractRaw(
      address,
      Buffer.from('636F6E74726163745F696E666F', 'hex'),
    )

    let parsedName = ''
    try {
      let details = JSON.parse(
        new TextDecoder().decode(contractName || undefined),
      )
      parsedName = details.contract
    } catch (e) {
      console.log('Error parsing contract name', e)
    }

    return {
      contractName: parsedName,
      contractAddress: address,
      contractCodeId: code_id,
      contractHistory: contractHistory,
      contract: contract,
      code: code,
    }
  }

  getQueryClient() {
    if (!this.queryClient) throw new Error('Query client not initialized')
    return this.queryClient
  }

  getVaultQueryClient() {
    let qClient = this.getQueryClient()
    return new BasicVaultQueryClient(qClient, VAULT_ADDRESS)
  }

  async getVaultCap() {
    let vault = this.getVaultQueryClient()
    return vault.getCap()
  }

  async getVaultTokenInfo() {
    let vault = this.getVaultQueryClient()
    return vault.tokenInfo()
  }

  async getVaultInvestmentInfo() {
    let vault = this.getVaultQueryClient()
    return vault.investment()
  }

  getPrimitiveQueryClient(address: string) {
    let qClient = this.getQueryClient()
    return new LpStrategyQueryClient(qClient, address)
  }

  async getPrimitiveLockStatus(address: string) {
    let primitive = this.getPrimitiveQueryClient(address)
    return primitive.lock()
  }

  async getPrimitivePendingAcks(address: string) {
    let primitive = this.getPrimitiveQueryClient(address)
    return primitive.listPendingAcks()
  }

  async getPrimitiveTrappedErrors(address: string) {
    let primitive = this.getPrimitiveQueryClient(address)
    return primitive.trappedErrors()
  }

  async getPrimitiveIcaAddress(address: string) {
    let primitive = this.getPrimitiveQueryClient(address)
    return primitive.icaAddress()
  }
}
