import * as chalk from 'chalk'
import * as fs from 'fs'
import { ContractMetadata, useAppContext } from '../../context/ScreenContext'
import { getEnv, getArgsForUploadType } from '../../utils/commandUtils'
import {
  getContractDirectory,
  getCWD,
  getDefaultForCodeId,
  saveMeta
} from '../../utils/fileUtils'

const ContractWindow = ({
  contractDetails
}: {
  contractDetails?: {
    wasm: fs.Stats | null
    optimized: fs.Stats | null
  }
}) => {
  const {
    contract,
    setContract,
    setCodeId,
    setCommand,
    env,
    log
  } = useAppContext()

  function handleBuildWasm () {
    setCommand({
      command: 'cargo',
      args: ['build', '--release', '--target', 'wasm32-unknown-unknown'],
      cwd: getContractDirectory(contract!.fileName), // always defined here
      env: {
        RUSTFLAGS: '-C link-arg=-s'
      }
    })
  }

  function handleBuildOptimized () {
    const path = getCWD()
    setCommand({
      command: 'docker', //run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6
      args: [
        'run',
        '--rm',
        '-v',
        `${path}:/code`,
        '--mount',
        `type=volume,source=${
          path.split('/')[path.split('/').length - 1]
        }_cache,target=/code/target`,
        '--mount',
        'type=volume,source=registry_cache,target=/usr/local/cargo/registry',
        'cosmwasm/rust-optimizer:0.12.6'
      ],
      cwd: path
    })
  }

  function uploadCallback (output: string) {
    if (!output) {
      log('No output from upload command.')
      return
    }
    const events = JSON.parse(output.split('\n')[1]).logs[0].events
    const codeID = events[events.length - 1].attributes[0].value as string

    if (!codeID) {
      //assume something went wrong
      log(
        "Couldn't find codeId in output. Please check the output and try again."
      )
      return
    }

    log(chalk.bold(chalk.red('CODE ID: ')) + codeID)

    // update contract metadata with new code
    const updatedContract: ContractMetadata = {
      ...contract!,
      codes: [...contract!.codes, getDefaultForCodeId(codeID)]
    }
    setContract(updatedContract)
    // set codeId to be active
    setCodeId(codeID)

    // save our new contract metadata
    saveMeta(updatedContract, env)
  }

  function handleUploadWasm () {
    const envConfig = getEnv(env, log)

    // contract always defined here
    setCommand({
      command: envConfig.command,
      args: getArgsForUploadType('wasm', {
        contract: contract!.fileName,
        envConfig
      }),
      cwd: getCWD(),
      callback: uploadCallback
    })
  }

  function handleUploadOpt () {
    const envConfig = getEnv(env, log)

    // contract always defined here
    setCommand({
      command: envConfig.command,
      args: getArgsForUploadType('optimized', {
        contract: contract!.fileName,
        envConfig
      }),
      cwd: getCWD(),
      callback: uploadCallback
    })
  }

  const detailTextLines = [
    'Last WASM build:',
    contractDetails?.wasm ? contractDetails.wasm.mtime.toDateString() : '',
    contractDetails?.wasm ? contractDetails.wasm.mtime.toTimeString() : '',
    '',
    'Last optimized build:',
    contractDetails?.optimized
      ? contractDetails.optimized.mtime.toDateString()
      : '',
    contractDetails?.optimized
      ? contractDetails.optimized.mtime.toTimeString()
      : ''
  ]

  const detailText = detailTextLines.join('\n')

  return (
    <>
      <text top={1} left={1}>
        {detailText}
      </text>
      {contractDetails?.wasm && (
        <button
          border={{ type: 'line' }}
          top={1}
          height={3}
          width={15}
          right={1}
          mouse
          // @ts-ignore
          onPress={handleUploadWasm}
        >
          Upload WASM
        </button>
      )}
      {contractDetails?.optimized && (
        <button
          border={{ type: 'line' }}
          top={6}
          height={3}
          width={15}
          right={1}
          mouse
          // @ts-ignore
          onPress={handleUploadOpt}
        >
          Upload Opt
        </button>
      )}
      <button
        top={detailTextLines.length + 2}
        left={1}
        height={3}
        width={18}
        border={{ type: 'line' }}
        mouse
        // @ts-ignore
        onPress={handleBuildWasm}
      >
        Build WASM
      </button>
      <button
        top={detailTextLines.length + 2}
        left={1 + 18 + 1}
        height={3}
        width={18}
        border={{ type: 'line' }}
        mouse
        // @ts-ignore
        onPress={handleBuildOptimized}
      >
        Build Optimized
      </button>
    </>
  )
}

export default ContractWindow
