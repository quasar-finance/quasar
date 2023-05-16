import * as chalk from 'chalk'
import * as fs from 'fs'
import { useEffect, useState } from 'react'
import { ContractMetadata, Tabs, useAppContext } from '../context/ScreenContext'
import { getCWD, loadMeta } from '../utils/fileUtils'

const Tabs = ({}) => {
  const {
    width,
    height,
    tab,
    setTab,
    setCommand,
    log,
    investmentInfo,
  } = useAppContext()
  const [err, setErr] = useState('')

  let [tabs, setTabs] = useState([] as Tabs[])

  useEffect(() => {
    if (investmentInfo) {
      setTabs(investmentInfo.primitives.map((_, i) => ('prim_' + i) as Tabs))
    }
  }, [investmentInfo])

  return (
    <box
      label=" contracts "
      border={{ type: 'line' }}
      top={0}
      height={'100%'}
      width={'100%'}
    >
      <button
        key={'vault'}
        top={0}
        height={3}
        width={15}
        border={{ type: 'line' }}
        mouse
        // @ts-ignore
        onPress={() => setTab('vault')}
      >
        vault
      </button>

      {tabs.length === 0 && (
        <text left={15} top={1}>
          Loading Primitives...
        </text>
      )}
      {tabs.map((tabName, i) => {
        const selected = tab === tabName ? '**' : ''
        return (
          <button
            key={tabName}
            top={0}
            height={3}
            width={15}
            left={15 * (i + 1)}
            border={{ type: 'line' }}
            mouse
            // @ts-ignore
            onPress={() => setTab(tabName as Tabs)}
          >
            <text>{`${selected} ${tabName} ${selected}`}</text>
          </button>
        )
      })}
    </box>
  )
}

export default Tabs
