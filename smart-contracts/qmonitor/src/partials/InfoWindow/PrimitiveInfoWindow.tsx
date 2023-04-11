import { useEffect, useState } from 'react'
import Querier from '../../chain/Querier'
import { QUASAR_RPC_NODE } from '../../utils/config'
import { useAppContext } from '../../context/ScreenContext'
import { TPosition } from '../../utils/types'
import { assert } from 'console'
import { InstantiateMsg } from '../../contracts/BasicVault.types'
import { OsmosisClient } from '../../chain/Osmosis'
import chalk = require('chalk')
import { QuasarClient } from '../../chain/Quasar'

const PrimitiveInfoWindow = ({ height }: { height: TPosition }) => {
  const { tab, investmentInfo } = useAppContext()

  const [lockStatus, setLockStatus] = useState('loading...')
  const [pendingAcks, setPendingAcks] = useState('loading...')
  const [trappedErrors, setTrappedErrors] = useState('loading...')
  const [quasarBalance, setQuasarBalance] = useState('loading...')
  const [osmoAddress, setOsmoAddress] = useState('loading...')
  const [osmoBalance, setOsmoBalance] = useState('loading...')
  const [osmoLockedShares, setOsmoLockedShares] = useState('loading...')

  let pidx = Number.parseInt(tab.split('_')[1])

  assert(pidx >= 0)

  if (!investmentInfo) {
    throw new Error('investmentInfo is not loaded')
  }

  async function loadIcaAddress() {
    let querier = await Querier.getInstance(QUASAR_RPC_NODE, 'quasar-1')
    let icaAddress = await querier.getPrimitiveIcaAddress(
      investmentInfo!.primitives[pidx].address,
    )
    setOsmoAddress(icaAddress.address)
  }

  async function loadOsmoBalance() {
    let querier = await OsmosisClient.getInstance()
    let balance = await querier.getBalances(osmoAddress)
    setOsmoBalance(balance.balances.map((b) => b.amount + b.denom).join(', '))
  }

  async function loadOsmoLockedShares() {
    let querier = await OsmosisClient.getInstance()
    let lockedShares = await querier.getLockedShares(osmoAddress)
    setOsmoLockedShares(
      lockedShares.coins.map((b) => b.amount + b.denom).join(', '),
    )
  }

  async function loadQuasarBalances() {
    let querier = await QuasarClient.getInstance()
    let balances = await querier.getBalances(
      investmentInfo!.primitives[pidx].address,
    )

    setQuasarBalance(balances.map((b) => b.amount + b.denom).join(', '))
  }

  async function loadLockedStatus() {
    let querier = await Querier.getInstance(QUASAR_RPC_NODE, 'quasar-1')
    let status = await querier.getPrimitiveLockStatus(
      investmentInfo!.primitives[pidx].address,
    )
    let lockType = Object.keys(status.lock).find((l) => status.lock[l])
    setLockStatus(
      lockType ? chalk.red(`locked (${lockType})`) : chalk.green('unlocked'),
    )
  }

  async function loadPendingAcks() {
    let querier = await Querier.getInstance(QUASAR_RPC_NODE, 'quasar-1')
    let pendingAcks = await querier.getPrimitivePendingAcks(
      investmentInfo!.primitives[pidx].address,
    )
    setPendingAcks(Object.keys(pendingAcks.pending).length.toString())
  }

  async function loadTrappedErrors() {
    let querier = await Querier.getInstance(QUASAR_RPC_NODE, 'quasar-1')
    let trappedErrors = await querier.getPrimitiveTrappedErrors(
      investmentInfo!.primitives[pidx].address,
    )
    setTrappedErrors(Object.keys(trappedErrors.errors).length.toString())
  }

  function pollPrimitiveState() {
    let interval = setInterval(() => {
      loadLockedStatus()
      loadPendingAcks()
      loadTrappedErrors()
    }, 5000)

    return () => {
      clearInterval(interval)
    }
  }

  useEffect(() => {
    // reset state when tab changes
    setLockStatus('loading...')
    setPendingAcks('loading...')
    setTrappedErrors('loading...')
    setQuasarBalance('loading...')
    setOsmoAddress('loading...')
    setOsmoBalance('loading...')
    setOsmoLockedShares('loading...')

    return pollPrimitiveState()
  }, [tab])

  async function loadOsmoDetails() {
    try {
      await loadOsmoBalance()
      await loadOsmoLockedShares()
      await loadQuasarBalances()
    } catch (e) {
      console.error('OOF:', osmoAddress, e)
    }
  }

  useEffect(() => {
    loadIcaAddress()
      .then(() => OsmosisClient.getInstance())
      .catch((e) => {
        console.error(e)
      })
    if (osmoAddress === 'loading...') return
    loadOsmoDetails()
  }, [osmoAddress, tab])

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
        {`qsr address: ${investmentInfo.primitives[pidx].address}`}
      </text>
      <text left={2} top={2}>
        {`weight in vault: ${investmentInfo.primitives[pidx].weight}`}
      </text>

      <text left={2} top={4}>
        {`osmo address: ${osmoAddress}`}
      </text>
      <text left={2} top={5}>
        {`osmo balance: ${osmoBalance}`}
      </text>
      <text left={2} top={6}>
        {`osmo locked shares: ${osmoLockedShares}`}
      </text>
      <text left={2} top={7}>
        {`quasar balance: ${quasarBalance}`}
      </text>

      <text left={2} top={9}>
        {`lock status: ${lockStatus}`}
      </text>
      <text left={2} top={10}>
        {`pending acks: ${pendingAcks}`}
      </text>
      <text left={2} top={11}>
        {`trapped errors: ${trappedErrors}`}
      </text>

      {Object.keys(investmentInfo.primitives[pidx].init.l_p).map((k, i) => {
        return (
          <text left={2} top={13 + i} key={k + i}>
            {`${k}: ${
              investmentInfo.primitives[pidx].init.l_p[
                k as keyof InstantiateMsg
              ]
            }`}
          </text>
        )
      })}
    </box>
  )
}

export default PrimitiveInfoWindow
