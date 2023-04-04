import { useEffect, useRef, useState } from 'react'

const Counter = (props: any) => {
  const [count, setCount] = useState(0)
  const [fontIndex, setFontIndex] = useState(0)

  const boxRef = useRef(null)

  useEffect(() => {
    const timer = setTimeout(() => {
      setFontIndex(fontIndex + 1)
      // @ts-ignore
      boxRef?.current.setScrollPerc(100)
    }, 1000)
    return () => clearTimeout(timer)
  }, [fontIndex])

  function handleCountButton () {
    console.log('HERE!')
    setCount(count + 1)
  }

  const text = `You clicked ${count}\nstupid times, ${new Array(fontIndex)
    .fill(`\nI love you ${fontIndex}`)
    .join('')}`
  return (
    <>
      <box
      mouse
      ref={boxRef}
        scrollable
        style={{
          scrollbar: {
            bg: 'blue',
            fg: 'red',
            track: {
              bg: 'cyan',
              fg: 'magenta'
            }
          }
        }}
        bottom={1}
        height={5}
      >
        {text}
      </box>
      <button
        top={2}
        width={12}
        height={3}
        border={{ type: 'line' }}
        mouse
        // @ts-ignore
        onPress={handleCountButton}
      >
        Click me
      </button>
    </>
  )
}

export default Counter
