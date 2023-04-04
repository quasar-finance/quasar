import * as fs from 'fs'
import { useEffect, useState } from 'react'
import { useAppContext } from '../context/ScreenContext'
import { getContractDetails } from '../utils/fileUtils'
import { TPosition } from '../utils/types'
import { getCenterColWidth } from '../utils/windowUtils'
import CodeWindow from './MsgWindow/CodeWindow'
import ContractInstanceWindow from './MsgWindow/ContractInstanceWindow'
import ContractWindow from './MsgWindow/ContractWindow'

const MsgWindow = ({ height }: { height: TPosition }) => {
  const {
    contract,
    setContract,
    codeId,
    setCodeId,
    contractInstanceAddress,
    command,
    setCommand,
    width,
    env,
    log
  } = useAppContext()
  const [contractDetails, setContractDetails] = useState<{
    wasm: fs.Stats | null
    optimized: fs.Stats | null
  }>()

  useEffect(() => {
    if (contract) {
      try {
        setContractDetails(getContractDetails(contract.buildName))
      } catch (e) {
        console.log(e)
      }
    }
  }, [contract, command])

  // const centerColWidth = getCenterColWidth(width as number)
  // // const buttonLeft = Math.max(centerColWidth * 0.3, 20)
  // const buttonLeft = Math.floor(Math.max(centerColWidth * 0.45, 20))

  function renderActiveWindow () {
    if (contract && !codeId)
      return <ContractWindow contractDetails={contractDetails} />
    else if (codeId && !contractInstanceAddress) return <CodeWindow />
    else if (contractInstanceAddress) return <ContractInstanceWindow />
  }

  return (
    <box
      label={contract?.fileName || ' msg '}
      border={{ type: 'line' }}
      top={0}
      height={height}
      width={'100%'}
    >
      {renderActiveWindow()}
    </box>
  )
}

//└
export default MsgWindow
