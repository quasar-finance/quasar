import * as chalk from 'chalk'
import { useState } from 'react'
import { useAppContext } from '../../context/ScreenContext'
import ExecuteMsgTab from './ExecuteMsgTab'
import QueryMsgTab from './QueryMsgTab'

const ContractInstanceWindow = () => {
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

  const [activeTab, setActiveTab] = useState<'execute' | 'query' | 'none'>('none')

  function renderActiveTab () {
    if (activeTab === 'execute') {
      return <ExecuteMsgTab />
    } else if (activeTab === 'query') {
        return <QueryMsgTab />
    }
  }

  return (
    <box top={0} left={0}>
      <box top={0} left={0} height={3}>
        <button
          top={0}
          left={0}
          width={'50%'}
          height={3}
          border={{ type: 'line' }}
          mouse
          // @ts-ignore
          onPress={() => setActiveTab('execute')}
        >
          {activeTab === 'execute' ? chalk.inverse(`Execute`) : 'Execute'}
        </button>
        <button
          top={0}
          right={0}
          width={'50%-1'}
          height={3}
          border={{ type: 'line' }}
          mouse
          // @ts-ignore
          onPress={() => setActiveTab('query')}
        >
          {activeTab === 'query' ? chalk.inverse(`Query`) : 'Query'}
        </button>
      </box>
      <box top={3} left={0}>
        {renderActiveTab()}
      </box>
    </box>
  )
}

export default ContractInstanceWindow
