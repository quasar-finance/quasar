import { OSMOSIS_RPC_NODE } from '../utils/config'
import { osmosis } from 'osmojs'

const { createRPCQueryClient } = osmosis.ClientFactory

let instance: OsmosisClient | undefined = undefined

export class OsmosisClient {
  rpcUrl: string
  constructor() {
    this.rpcUrl = OSMOSIS_RPC_NODE
  }

  static async getInstance(): Promise<OsmosisClient> {
    if (!instance) {
      instance = new OsmosisClient()
      await instance.init()
    }
    return instance
  }

  async init() {
    const client = await createRPCQueryClient({ rpcEndpoint: this.rpcUrl })
  }

  async getPoolInfo(poolId: string) {
    const client = await createRPCQueryClient({ rpcEndpoint: this.rpcUrl })
    const response = await client.osmosis.gamm.v1beta1.pool({
      poolId,
    })
    if (!response.pool) {
      throw new Error('Pool not found')
    }

    return osmosis.gamm.v1beta1.Pool.decode(response.pool.value)
  }

  async getBalances(address: string) {
    const client = await createRPCQueryClient({ rpcEndpoint: this.rpcUrl })
    const balance = await client.cosmos.bank.v1beta1.allBalances({ address })
    return balance
  }

  async getLockedShares(address: string) {
    const client = await createRPCQueryClient({ rpcEndpoint: this.rpcUrl })
    const response = await client.osmosis.lockup.accountLockedCoins({
      owner: address,
    })

    return response
  }
}
