import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { StargateClient } from '@cosmjs/stargate'
import { QUASAR_RPC_NODE } from '../utils/config'

let instance: QuasarClient | undefined = undefined

export class QuasarClient {
  rpcUrl: string
  constructor() {
    this.rpcUrl = QUASAR_RPC_NODE
  }

  static getInstance(): QuasarClient {
    if (!instance) {
      instance = new QuasarClient()
      //   await instance.init()
    }
    return instance
  }
  async init() {
    // const client = await CosmWasmClient.connect(this.rpcUrl)
  }

  async getBalances(address: string) {
    const client = await StargateClient.connect(this.rpcUrl)
    const balance = await client.getAllBalances(address)
    return balance
  }
}
