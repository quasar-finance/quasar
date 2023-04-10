import { useEffect, useState } from 'react'
import Querier from '../chain/Querier'
import { QUASAR_RPC_NODE } from '../utils/config'
import { useAppContext } from '../context/ScreenContext'
import { TPosition } from '../utils/types'
import VaultInfoWindow from './InfoWindow/VaultInfoWindow'
import PrimitiveInfoWindow from './InfoWindow/PrimitiveInfoWindow'

const InfoWindow = ({ height }: { height: TPosition }) => {
  const { tab, investmentInfo } = useAppContext()

  return (
    <>
      {!tab.startsWith('prim') ? (
        <VaultInfoWindow height={height} />
      ) : (
        <PrimitiveInfoWindow height={height} />
      )}
    </>
  )
}

export default InfoWindow
