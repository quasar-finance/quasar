import * as chalk from 'chalk'
import { useEffect, useRef, useState } from 'react'
import { MsgMetadata, useAppContext } from '../../context/ScreenContext'
import { getEnv } from '../../utils/commandUtils'

// GenericMsgTab can execute arbitrary init/execute/query messages given the right params
const GenericMsgTab = ({
  label,
  type,
  savedMsgs,
  includeFunds,
  saveMsg,
  deleteMsg,
  sendMsg
}: {
  label: string
  type: 'init' | 'execute' | 'query'
  savedMsgs: MsgMetadata[]
  includeFunds?: boolean
  saveMsg: (msg: MsgMetadata, i: number | null) => void
  deleteMsg: (index: number) => void
  sendMsg: (msg: MsgMetadata) => void
}) => {
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

  const [view, setView] = useState<'saved-msgs' | 'new-msg'>(
    savedMsgs.length ? 'saved-msgs' : 'new-msg'
  )
  const [activeMsg, setActiveMsg] = useState<number | null>(null)

  const msgTitle = useRef(null)
  const input = useRef(null)
  const funds = useRef(null)

  const saveMsgWrapper = () => {
    // @ts-ignore
    const title = msgTitle.current.getValue()
    // @ts-ignore
    const msg = input.current.getValue()
    // @ts-ignore
    const funds = funds.current.getValue()

    log(`saving new msg: ${title}`)

    if (contract && contract.fileName && codeId && env) {
      saveMsg(
        {
          title,
          msg,
          funds
        },
        activeMsg
      )
      setView('saved-msgs')
      setActiveMsg(null)
    } else {
      log('error saving msg: contract, codeId, or env not set for some reason')
    }
  }

  const deleteMsgWrapper = (i: number) => {
    if (contract && contract.fileName && codeId && env) {
      deleteMsg(i)
    } else {
      log(
        'error deleting msg: contract, codeId, or env not set for some reason'
      )
    }
  }

  const sendMsgWrapper = (i: number) => {
    if (contract && contract.fileName && codeId && env) {
      sendMsg(savedMsgs[i])
    } else {
      log('error sending msg: contract, codeId, or env not set for some reason')
    }
  }

  useEffect(() => {
    try {
      if (funds.current) {
        // @ts-ignore
        funds.current.key(['escape', 'C-c'], () => {
          // @ts-ignore
          funds.current.cancel()
        })

        // @ts-ignore
        funds.current.setValue('0')
      }
      if (msgTitle.current && input.current) {
        // @ts-ignore
        msgTitle.current.setValue(`${contract?.fileName}'s ${type} message`)
        // @ts-ignore
        msgTitle.current.key(['escape', 'C-c'], () => {
          // @ts-ignore
          msgTitle.current.cancel()
        })
        // @ts-ignore
        msgTitle.current.key(['tab'], () => {
          // @ts-ignore
          msgTitle.current.cancel()
          // @ts-ignore
          input.current.focus()
        })

        // @ts-ignore
        input.current.key(['escape', 'C-c'], () => {
          // @ts-ignore
          input.current.cancel()
        })
      }
    } catch (e) {
      console.error(e)
    }
  }, [])

  useEffect(() => {
    if (activeMsg !== null && view === 'new-msg') {
      const msg = savedMsgs[activeMsg]
      log(`loading msg ${activeMsg}: ${msg.title}`)
      // @ts-ignore
      msgTitle.current.setValue(msg.title)
      // @ts-ignore
      input.current.setValue(msg.msg)
    }
  }, [activeMsg, view])

  return (
    <box top={0} left={0}>
      <text left={1}>
        {chalk.bold(chalk.bgGreen(`${label}`)) +
          chalk.gray(' (press esc if stuck)')}
      </text>
      {view === 'new-msg' ? (
        <>
          <textbox
            label={chalk.green(' msg title (for saving)')}
            top={1}
            height={3}
            border={{ type: 'line' }}
            ref={msgTitle}
            keys
            inputOnFocus
            mouse
          />
          <textarea
            label={chalk.green(' msg value (valid json) ')}
            ref={input}
            inputOnFocus
            top={4}
            height={6}
            width={includeFunds ? '70%-1' : '100%-1'}
            keys
            mouse
            border={{ type: 'line' }}
          />
          {includeFunds && (
            <textarea
              label={chalk.green(' funds ')}
              ref={funds}
              inputOnFocus
              top={4}
              height={6}
              left={'70%'}
              border={{ type: 'line' }}
              keys
              mouse
            />
          )}
          <button
            top={10}
            height={3}
            width={'50%-1'}
            border={{ type: 'line' }}
            mouse
            // @ts-ignore
            onPress={() => setView('saved-msgs')}
          >
            Go to saved msgs
          </button>
          <button
            top={10}
            height={3}
            width={'50%-1'}
            right={0}
            border={{ type: 'line' }}
            mouse
            // @ts-ignore
            onPress={saveMsgWrapper}
          >
            Save this message
          </button>
        </>
      ) : (
        <>
          {savedMsgs.map((msg: MsgMetadata, i: number) => {
            return (
              <box key={i} top={i * 3 + 1} height={3} width={'100%'}>
                <button
                  top={0}
                  height={3}
                  width={'60%'}
                  border={{ type: 'line' }}
                  mouse
                  // @ts-ignore
                  onPress={() => {
                    setActiveMsg(i)
                    setView('new-msg')
                  }}
                >
                  {msg.title}
                </button>
                <button
                  top={0}
                  height={3}
                  width={'15%'}
                  left={'60%+1'}
                  border={{ type: 'line' }}
                  mouse
                  // @ts-ignore
                  onPress={() => deleteMsgWrapper(i)}
                >
                  {` X `}
                </button>
                <button
                  top={0}
                  height={3}
                  width={'25%'}
                  right={0}
                  border={{ type: 'line' }}
                  mouse
                  // @ts-ignore
                  onPress={() => sendMsgWrapper(i)}
                >
                  {` ${type.toLocaleUpperCase()} `}
                </button>
              </box>
            )
          })}
          <button
            right={0}
            width={20}
            bottom={0}
            height={3}
            border={{ type: 'line' }}
            mouse
            // @ts-ignore
            onPress={() => setView('new-msg')}
          >
            Add new Msg
          </button>
        </>
      )}
    </box>
  )
}

export default GenericMsgTab
