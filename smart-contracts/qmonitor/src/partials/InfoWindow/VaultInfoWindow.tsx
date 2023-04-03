import { useEffect, useState } from 'react'
import Querier from '../../chain/Querier'
import { QUASAR_RPC_NODE } from '../../utils/config'
import { useAppContext } from '../../context/ScreenContext'
import { TPosition } from '../../utils/types'

const VaultInfoWindow = ({ height }: { height: TPosition }) => {
  const { tab, investmentInfo } = useAppContext()

  const [cap, setCap] = useState('loading...')
  const [totalCap, setTotalCap] = useState('loading...')
  const [capAdmin, setCapAdmin] = useState('loading...')
  const [name, setName] = useState('loading...')
  const [symbol, setSymbol] = useState('loading...')
  const [decimals, setDecimals] = useState('loading...')
  const [totalSupply, setTotalSupply] = useState('loading...')

  async function loadCap() {
    let querier = await Querier.getInstance(QUASAR_RPC_NODE, 'quasar-1')
    let vcap = await querier.getVaultCap()
    setCap(vcap.cap.current)
    setTotalCap(vcap.cap.total)
    setCapAdmin(vcap.cap.cap_admin)
  }

  async function loadTokenInfo() {
    let querier = await Querier.getInstance(QUASAR_RPC_NODE, 'quasar-1')
    let tokenInfo = await querier.getVaultTokenInfo()
    setName(tokenInfo.name)
    setSymbol(tokenInfo.symbol)
    setDecimals(tokenInfo.decimals.toString())
    setTotalSupply(tokenInfo.total_supply)
  }

  useEffect(() => {
    loadCap()
    setTimeout(() => loadTokenInfo(), 700)
  }, [])

  return (
    <box
      label={` ${tab} `}
      border={{ type: 'line' }}
      top={0}
      right={0}
      height={height}
      width={'100%'}
    >
      <text left={2} top={1}>
        {`current cap: ${cap}`}
      </text>
      <text left={2} top={2}>
        {`cap limit: ${totalCap}`}
      </text>
      <text left={2} top={3}>
        {`cap admin: ${capAdmin}`}
      </text>

      <text left={2} top={5}>
        {`name: ${name}`}
      </text>
      <text left={2} top={6}>
        {`symbol: ${symbol}`}
      </text>
      <text left={2} top={7}>
        {`decimals: ${decimals}`}
      </text>
      <text left={2} top={8}>
        {`total supply: ${totalSupply}`}
      </text>
    </box>
  )
}

export default VaultInfoWindow
