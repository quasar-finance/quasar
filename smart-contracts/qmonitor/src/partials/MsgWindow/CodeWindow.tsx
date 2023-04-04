import * as chalk from 'chalk'
import { useEffect, useRef, useState } from 'react'
import { DetailedBlessedProps, TextareaElement } from 'react-blessed'
import {
  ContractMetadata,
  CodeMetadata,
  useAppContext,
  MsgMetadata
} from '../../context/ScreenContext'
import { getEnv } from '../../utils/commandUtils'
import { getActiveCode, getCWD } from '../../utils/fileUtils'

const CodeWindow = () => {
  const {
    contract,
    setContract,
    codeId,
    setCodeId,
    setContractInstanceAddress,
    command,
    setCommand,
    width,
    env,
    log
  } = useAppContext()

  const [view, setView] = useState<'saved-msgs' | 'new-msg'>(
    contract?.initMsgs.length ? 'saved-msgs' : 'new-msg'
  )
  const [activeMsg, setActiveMsg] = useState<number | null>(null)

  const msgTitle = useRef(null)
  const input = useRef(null)
  const funds = useRef(null)

  const saveInitMsg = () => {
    // @ts-ignore
    const title = msgTitle.current.getValue()
    // @ts-ignore
    const msg = input.current.getValue()
    // @ts-ignore
    const funds = funds.current.getValue()

    log(`saving new msg: ${title}`)

    if (contract && contract.fileName && codeId && env) {
      setContract({
        ...contract,
        initMsgs:
          activeMsg === null
            ? [
                ...contract.initMsgs,
                {
                  title,
                  msg,
                  funds
                }
              ]
            : contract.initMsgs.map((m, i) => {
                if (i === activeMsg) return { title, msg, funds }
                return m
              })
      })

      setView('saved-msgs')
      setActiveMsg(null)
    } else {
      log('error saving msg: contract, codeId, or env not set for some reason')
    }
  }

  const deleteMsg = (i: number) => {
    if (contract && contract.fileName && codeId && env) {
      setContract({
        ...contract,
        initMsgs: contract.initMsgs.filter((_, j) => j !== i)
      })
    } else {
      log(
        'error deleting msg: contract, codeId, or env not set for some reason'
      )
    }
  }

  function listContractsByCodeCallback (output: string) {
    if (!output) return
    log(output)

    const contracts = JSON.parse(output.split('\n')[0]).contracts
    const contractAddress = contracts[contracts.length - 1]

    log(chalk.bold(chalk.red('deployed contract address: ' + contractAddress)))
    setContract({
      ...contract!,
      codes: contract!.codes.map(codeMeta => {
        if (codeMeta.codeID === codeId) {
          return {
            ...codeMeta,
            deployedContracts: [
              ...codeMeta.deployedContracts,
              {
                address: contractAddress,
                executeMsgs: [],
                queryMsgs: []
              }
            ]
          }
        }
        return codeMeta
      })
    })
    setContractInstanceAddress(contractAddress)
  }

  function initCallback (_output: string) {
    if (!_output) return
    // log("here23")
    // var stack = new Error().stack
    // log(stack)
    // log(_output)
    log('init complete, fetching contract address...')
    const envConfig = getEnv(env)

    setCommand({
      command: envConfig.command,
      args: [
        'query',
        'wasm',
        'list-contract-by-code',
        codeId,
        '--output',
        'json',
        '--node',
        envConfig.node
      ],
      cwd: getCWD(),
      callback: listContractsByCodeCallback
    })
  }

  const sendInitMsg = (i: number) => {
    if (contract && contract.fileName && codeId && env) {
      const msg = contract.initMsgs[i]
      const envConfig = getEnv(env)
      const funds = msg.funds 

      setCommand({
        command: envConfig.command,
        args: [
          'tx',
          'wasm',
          'instantiate',
          codeId,
          msg.msg,
          '--from',
          envConfig.keyName,
          '--label',
          msg.title,
          '--gas-prices',
          envConfig.feeAmount + envConfig.feeDenom,
          '--gas',
          'auto',
          '--gas-adjustment',
          '1.3',
          '-b',
          'block',
          '-y',
          '--no-admin',
          '--node',
          envConfig.node,
          '--chain-id',
          envConfig.chainId,
          ...(funds ? ['--amount', funds] : [])
        ],
        cwd: getCWD(),
        callback: initCallback
      })
    } else {
      log('error sending msg: contract, codeId, or env not set for some reason')
    }
  }

  useEffect(() => {
    if (msgTitle.current && input.current && funds.current) {
      // @ts-ignore
      msgTitle.current.setValue(`${contract?.fileName}'s init message`)
      // @ts-ignore
      msgTitle.current.key(['escape', 'C-c'], () => {
        // @ts-ignore
        msgTitle.current.cancel()
      })

      // @ts-ignore
      funds.current.key(['escape', 'C-c'], () => {
        // @ts-ignore
        funds.current.cancel()
      })
      // @ts-ignore
      input.current.key(['escape', 'C-c'], () => {
        // @ts-ignore
        input.current.cancel()
      })
    }
  }, [])

  useEffect(() => {
    if (activeMsg !== null && view === 'new-msg') {
      const msg = contract!.initMsgs[activeMsg]
      log(`loading msg ${activeMsg}: ${msg.title}`)
      // @ts-ignore
      msgTitle.current.setValue(msg.title)
      // @ts-ignore
      input.current.setValue(msg.msg)
      // @ts-ignore
      funds.current.setValue(msg.funds)
    }
  }, [activeMsg, view])

  return (
    <box top={0} left={0}>
      <text left={1}>
        {chalk.bold(`Code ID: ${codeId} Init Msg`) +
          chalk.gray(' (press esc if stuck)')}
      </text>
      {view === 'new-msg' ? (
        <>
           <textbox
            label={chalk.green(' msg title (for saving)')}
            top={1}
            height={3}
            width={'60%-1'}
            border={{ type: 'line' }}
            ref={msgTitle}
            keys
            inputOnFocus
            mouse
          />
          <textarea
            label={chalk.green(' msg value (must be valid json) ')}
            ref={input}
            inputOnFocus
            top={4}
            height={6}
            width={'60%-1'}
            keys
            mouse
            border={{ type: 'line' }}
          />
          <textarea
            label={chalk.green(' funds (ex: 10uosmo) ')}
            ref={funds}
            inputOnFocus
            top={1}
            height={3}
            left={'60%'}
            border={{ type: 'line' }}
            keys
            mouse
          />
          <button
            top={10}
            height={3}
            width={'50%-1'}
            border={{ type: 'line' }}
            mouse
            // @ts-ignore
            onPress={() => setView('saved-msgs')}
          >
            Go to saved msgs
          </button>
          <button
            top={10}
            height={3}
            width={'50%-1'}
            right={0}
            border={{ type: 'line' }}
            mouse
            // @ts-ignore
            onPress={saveInitMsg}
          >
            Save this message
          </button>
        </>
      ) : (
        <>
          {contract?.initMsgs.map((msg: MsgMetadata, i: number) => {
            return (
              <box key={i} top={i * 3} height={3} width={'100%'}>
                <button
                  top={0}
                  height={3}
                  width={'60%'}
                  border={{ type: 'line' }}
                  mouse
                  // @ts-ignore
                  onPress={() => {
                    setActiveMsg(i)
                    setView('new-msg')
                  }}
                >
                  {msg.title}
                </button>
                <button
                  top={0}
                  height={3}
                  width={'15%'}
                  left={'60%+1'}
                  border={{ type: 'line' }}
                  mouse
                  // @ts-ignore
                  onPress={() => deleteMsg(i)}
                >
                  {` X `}
                </button>
                <button
                  top={0}
                  height={3}
                  width={'25%'}
                  right={0}
                  border={{ type: 'line' }}
                  mouse
                  // @ts-ignore
                  onPress={() => sendInitMsg(i)}
                >
                  {` Init `}
                </button>
              </box>
            )
          })}
          <button
            right={0}
            width={20}
            bottom={0}
            height={3}
            border={{ type: 'line' }}
            mouse
            // @ts-ignore
            onPress={() => setView('new-msg')}
          >
            Add new Msg
          </button>
        </>
      )}
    </box>
  )
}

export default CodeWindow
