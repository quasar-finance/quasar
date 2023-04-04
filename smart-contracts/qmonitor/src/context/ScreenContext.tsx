import {
  createContext,
  Dispatch,
  SetStateAction,
  useContext,
  useEffect,
  useState,
} from 'react'
import { TPosition } from '../utils/types'
import * as blessed from 'blessed'
import { saveMeta } from '../utils/fileUtils'
import { InvestmentInfo } from '../contracts/BasicVault.types'
import Querier from '../chain/Querier'
import { QUASAR_RPC_NODE } from '../utils/config'

function defaultSetter() {}

export interface SpawnCommand {
  command: string
  args: string[]
  cwd: string
  env?: { [key: string]: string }
  callback?: (output: string) => void
}

export interface MsgMetadata {
  title: string
  msg: string
  funds: string
}

export interface ContractInstanceMetadata {
  address: string
}

export interface CodeMetadata {
  codeID: string
  deployedContracts: ContractInstanceMetadata[]
}

export interface ContractMetadata {
  fileName: string
  buildName: string // same as filename generally but with underscores
  codes: CodeMetadata[]
  initMsgs: MsgMetadata[]
  executeMsgs: MsgMetadata[]
  queryMsgs: MsgMetadata[]
}

export type Tabs = 'vault' | 'prim_0' | 'prim_1' | 'prim_2'

const AppContext = createContext({
  width: 0 as TPosition,
  height: 0 as TPosition,
  tab: 'vault' as Tabs,
  investmentInfo: undefined as InvestmentInfo | undefined,
  command: undefined as SpawnCommand | undefined,
  logAppendContent: '' as string,
  setWidth: defaultSetter as Dispatch<SetStateAction<TPosition>>,
  setHeight: defaultSetter as Dispatch<SetStateAction<TPosition>>,
  setTab: defaultSetter as Dispatch<SetStateAction<Tabs>>,
  setCommand: defaultSetter as Dispatch<
    SetStateAction<SpawnCommand | undefined>
  >,
  log: (..._args: any[]) => {},
})

export function AppWrapper({
  screen,
  children,
}: {
  screen: blessed.Widgets.Screen
  children?: any
}) {
  const [width, setWidth] = useState<TPosition>(0)
  const [height, setHeight] = useState<TPosition>(0)
  const [tab, setTab] = useState<Tabs>('vault')
  const [command, setCommand] = useState<SpawnCommand | undefined>(undefined)
  const [logAppendContent, setLogAppendContent] = useState<string>('')
  const [investmentInfo, setInvestmentInfo] = useState<
    InvestmentInfo | undefined
  >(undefined)

  useEffect(() => {
    setWidth(screen.width)
    setHeight(screen.height)

    screen.on('resize', (newScreen) => {
      console.log({ newScreen })
      setWidth(screen.width)
      setHeight(screen.height)
    })
  }, [])

  async function loadInvesmentInfo() {
    const querier = await Querier.getInstance(QUASAR_RPC_NODE, 'quasar-1')
    const investmentResponse = await querier.getVaultInvestmentInfo()
    setInvestmentInfo(investmentResponse.info)
  }

  // vault loading primitives
  useEffect(() => {
    Querier.onInit(() => {
      loadInvesmentInfo()
    })
  }, [])

  // useEffect(() => {
  //   if (contract && contract.fileName && env) {
  //     saveMeta(contract, env)
  //   }
  // }, [contract])

  let sharedState = {
    width,
    setWidth,
    height,
    setHeight,
    tab,
    setTab,
    investmentInfo,
    command,
    setCommand,
    logAppendContent,
    log: (...args: string[]) => {
      setLogAppendContent(
        args
          .map((a) => {
            if (typeof a === 'object') return JSON.stringify(a)
            return a
          })
          .join(' ') + '\n',
      )
    },
  }

  return (
    <AppContext.Provider value={sharedState}>{children}</AppContext.Provider>
  )
}

export function useAppContext() {
  return useContext(AppContext)
}
