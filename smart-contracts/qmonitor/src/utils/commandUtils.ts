import { EnvConfig, envConfigs } from '../partials/EnvList'
import { getOptimizedArtifactPath, getWasmArtifactPath } from './fileUtils'

export interface UploadOptions {
  contract: string
  envConfig: EnvConfig
}

export function getEnv (env: string, log?: Function) {
  const envConfig = envConfigs.find(e => e.chainId === env)
  if (!envConfig) {
    if (log) log('No env found, please select one from the environment list')
    throw new Error('No env config found for uploading wasm')
  }

  return envConfig
}

export function getArgsForUploadType (
  uploadType: 'wasm' | 'optimized',
  uploadOpts: UploadOptions
) {
  const { contract, envConfig } = uploadOpts

  return [
    'tx',
    'wasm',
    'store',
    uploadType === 'wasm'
      ? getWasmArtifactPath(contract)
      : getOptimizedArtifactPath(contract),
    '--from',
    envConfig.keyName,
    '-y',
    '--output',
    'json',
    '-b',
    'block',
    '--node',
    envConfig.node,
    '--chain-id',
    'quasar',
    '--gas-prices',
    envConfig.feeAmount + envConfig.feeDenom,
    '--gas',
    'auto',
    '--gas-adjustment',
    '1.3'
  ]
}
