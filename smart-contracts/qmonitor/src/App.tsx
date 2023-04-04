import { useAppContext } from './context/ScreenContext'
import Tabs from './partials/Tabs'
import Debug from './partials/Debug'
import EnvList from './partials/EnvList'
import Logs from './partials/Logs'
import MsgWindow from './partials/MsgWindow'
import { IS_DEBUG, SIDEBAR_WIDTH } from './utils/config'
import { getCenterColWidth } from './utils/windowUtils'
import InfoWindow from './partials/InfoWindow'

// Rendering a simple centered box
const App = ({}) => {
  const { width, height } = useAppContext() as { width: number; height: number }

  const firstColWidth = Math.floor(Math.min(width * 0.3, SIDEBAR_WIDTH))
  const secondColWidth = getCenterColWidth(width)
  const lastColWidth = Math.floor(Math.min(width * 0.3, SIDEBAR_WIDTH))
  const secondColLeft = firstColWidth
  // const lastColLeft = firstColWidth + secondColWidth

  return (
    <element>
      {/* <box
          top='center'
          left='center'
          width='50%'
          height='50%'
          border={{ type: 'line' }}
          style={{ border: { fg: 'blue' } }}
        >
          
          <Counter />
          <text top={5}>hello govnahs</text>
        </box> */}
      {/* <Contracts maxWidth={40} width={'30%'} height={'100%'} /> */}
      <box width={'100%'} height={5}>
        <Tabs />
      </box>
      <box top={5} width={'100%'} height={height - 5}>
        <InfoWindow height={'100%'} />
      </box>
      {/* <box top={5} left={secondColLeft} width={secondColWidth} height={'100%'}>
        <MsgWindow height={'50%'} />
        <Logs top={'50%'} height={'50%'} />
      </box>
      <box right={0} width={lastColWidth} height={'100%'}>
        <EnvList height={IS_DEBUG ? height - 3 : height} />
        {IS_DEBUG && <Debug />}
      </box> */}
    </element>
  )
}

export default App
