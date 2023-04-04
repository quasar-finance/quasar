import { useState } from 'react'
import { MsgMetadata, useAppContext } from '../../context/ScreenContext'
import { getEnv } from '../../utils/commandUtils'
import { getCWD } from '../../utils/fileUtils'
import GenericMsgTab from './GenericMsgTab'

const QueryMsgTab = () => {
  const {
    contract,
    setContract,
    codeId,
    setCodeId,
    contractInstanceAddress,
    setContractInstanceAddress,
    command,
    setCommand,
    width,
    env,
    log
  } = useAppContext()

  function savequeryMsg (msg: MsgMetadata, i: number | null) {
    setContract({
      ...contract!,
      queryMsgs:
        i === null
          ? [...contract!.queryMsgs, msg]
          : contract!.queryMsgs.map((m, j) => {
              if (j === i) return msg
              return m
            })
    })
  }

  function deletequeryMsg (i: number) {
    setContract({
      ...contract!,
      queryMsgs: contract!.queryMsgs.filter((_, j) => j !== i)
    })
  }

  function sendqueryMsg (msg: MsgMetadata) {
    const envConfig = getEnv(env)

    setCommand({
      command: envConfig.command,
      args: [
        'query',
        'wasm',
        'contract-state',
        'smart',
        contractInstanceAddress,
        msg.msg,
        '-o',
        'json',
        '--node',
        envConfig.node,
        '--chain-id',
        envConfig.chainId
      ],
      cwd: getCWD()
    })
  }

  return (
    <GenericMsgTab
      label={`Query Msg (${contractInstanceAddress.slice(
        0,
        6
      )}...${contractInstanceAddress.slice(-6)})`}
      type={'query'}
      savedMsgs={contract!.queryMsgs}
      saveMsg={savequeryMsg}
      deleteMsg={deletequeryMsg}
      sendMsg={sendqueryMsg}
    />
  )
}

export default QueryMsgTab
