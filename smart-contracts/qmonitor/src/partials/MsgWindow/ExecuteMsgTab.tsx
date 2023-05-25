import { useState } from 'react'
import { MsgMetadata, useAppContext } from '../../context/ScreenContext'
import { getEnv } from '../../utils/commandUtils'
import { getCWD } from '../../utils/fileUtils'
import GenericMsgTab from './GenericMsgTab'

const ExecuteMsgTab = () => {
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

  function saveExecuteMsg (msg: MsgMetadata, i: number | null) {
    setContract({
      ...contract!,
      executeMsgs:
        i === null
          ? [...contract!.executeMsgs, msg]
          : contract!.executeMsgs.map((m, j) => {
              if (j === i) return msg
              return m
            })
    })
  }

  function deleteExecuteMsg (i: number) {
    setContract({
      ...contract!,
      executeMsgs: contract!.executeMsgs.filter((_, j) => j !== i)
    })
  }

  function sendExecuteMsg (msg: MsgMetadata) {
    // quasarnoded tx wasm execute $ADDR "$MSG1" --from alice --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID
    const envConfig = getEnv(env)

    setCommand({
      command: envConfig.command,
      args: [
        'tx',
        'wasm',
        'execute',
        contractInstanceAddress,
        msg.msg,
        '--from',
        envConfig.keyName,
        '--gas-prices',
        envConfig.feeAmount + envConfig.feeDenom,
        '--gas',
        'auto',
        '--gas-adjustment',
        '1.3',
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
      label={`Execute Msg (${contractInstanceAddress.slice(
        0,
        6
      )}...${contractInstanceAddress.slice(-6)})`}
      type={'execute'}
      includeFunds={true}
      savedMsgs={contract!.executeMsgs}
      saveMsg={saveExecuteMsg}
      deleteMsg={deleteExecuteMsg}
      sendMsg={sendExecuteMsg}
    />
  )
}

export default ExecuteMsgTab
