import * as fs from 'fs'
import { execSync } from 'child_process'
import {
  CodeMetadata,
  ContractInstanceMetadata,
  ContractMetadata
} from '../context/ScreenContext'

export function getCWD () {
  const rootPath =
    '/Users/nikitajerschow/Documents/PassiveIncome/CryptoBase/QuasarBase/quasar/smart-contracts'
  return rootPath
  return process.cwd()
}

export function getFileDetails (filePath: string) {
  // find last modified date

  return fs.statSync(filePath)
}

export const getWasmArtifactPath = (contract: string) =>
  `${getCWD()}/target/wasm32-unknown-unknown/release/${formatWithUnderscores(
    contract
  )}.wasm`
export const getOptimizedArtifactPath = (contract: string) =>
  `${getCWD()}/artifacts/${formatWithUnderscores(contract)}.wasm`

export function getWasmArtifactsDetails (contract: string) {
  try {
    const filePath = getWasmArtifactPath(contract)
    return getFileDetails(filePath)
  } catch (e) {
    return null
  }
}

export function getOptimizedArtifactsDetails (contract: string) {
  try {
    const filePath = getWasmArtifactPath(contract)
    return getFileDetails(filePath)
  } catch (e) {
    return null
  }
}

export function getContractDetails (contract: string) {
  return {
    wasm: getWasmArtifactsDetails(contract),
    optimized: getOptimizedArtifactsDetails(contract)
  }
}

export function formatWithUnderscores (contract: string) {
  // just replace dashes with underscores
  return contract.replace(/-/g, '_')
}

export function getContractDirectory (contract: string) {
  return `${getCWD()}/contracts/${contract}`
}

export function getDefaultForContractFilename (
  contract: string
): ContractMetadata {
  return {
    fileName: contract,
    buildName: formatWithUnderscores(contract),
    codes: [],
    initMsgs: [],
    executeMsgs: [],
    queryMsgs: []
  }
}

export function getDefaultForCodeId (codeID: string): CodeMetadata {
  return {
    codeID,
    deployedContracts: []
  }
}

export function getDefaultForContractInstance (
  address: string
): ContractInstanceMetadata {
  return {
    address
  }
}

function getEnvPath (env: string) {
  const cwd = getCWD()
  return `${cwd}/.cosmwander/${env}`
}

function loadEnvs () {
  const cwd = getCWD()
  const envs = fs.readdirSync(`${cwd}/.cosmwander`)
  return envs
}

function getContractPath (env: string, fname: string) {
  const cwd = getCWD()
  return `${cwd}/.cosmwander/${env}/contracts/${fname}.json`
}

export function createWanderStore (env?: string) {
  // create a store for the contracts (.cosmwander folder)
  // this is where we will store the contract names, codes, and addresses for different environments
  const cwd = getCWD()

  if (env) {
    if (!fs.existsSync(getEnvPath(env)))
      fs.mkdirSync(`${getEnvPath(env)}/contracts`, { recursive: true })
  } else {
    if (!fs.existsSync(`${cwd}/.cosmwander`)) fs.mkdirSync(`${cwd}/.cosmwander`)
  }
}

export function saveMeta (contractMeta: ContractMetadata, env: string) {
  // if .cosmwander doesn't exist, call initWanderStore to create it
  if (!fs.existsSync(`${getEnvPath(env)}/contracts`)) {
    createWanderStore(env)
  }

  const { fileName } = contractMeta
  const contractStateFilePath = getContractPath(env, fileName)

  if (!fs.existsSync(contractStateFilePath)) {
    // create the file
    fs.writeFileSync(contractStateFilePath, '')
  }

  fs.writeFileSync(contractStateFilePath, JSON.stringify(contractMeta, null, 2))
}

export function getActiveCode (
  contract: ContractMetadata,
  codeID: string
): CodeMetadata | undefined {
  return contract.codes.find(c => c.codeID === codeID)
}

export function saveCommandToHistory (command: string) {
  if (!fs.existsSync(`${getCWD()}/.cosmwander/cmd-history.log`)) {
    fs.writeFileSync(`${getCWD()}/.cosmwander/cmd-history`, '')
  }

  fs.appendFileSync(`${getCWD()}/.cosmwander/cmd-history.log`, `${command}\n`)
}

////// KEEP THIS FUNCTION LAST
export function loadMeta (
  contractFileName: string,
  env: string
): ContractMetadata {
  if (!env || !fs.existsSync(getContractPath(env, contractFileName)))
    return getDefaultForContractFilename(contractFileName)

  const contractStateFilePath = getContractPath(env, contractFileName)
  const contractMeta = JSON.parse(
    fs.readFileSync(contractStateFilePath, 'utf8')
  )

  // migrations start

  // initMsgs in contract root, added Nov 14 2022
  if (!contractMeta.initMsgs) {
    contractMeta.initMsgs = []
  }

  // executeMsgs and queryMsgs in contract root, added Nov 15 2022
  if (!contractMeta.executeMsgs) {
    contractMeta.executeMsgs = []
  }
  if (!contractMeta.queryMsgs) {
    contractMeta.queryMsgs = []
  }

  // migrations end

  return contractMeta
}
