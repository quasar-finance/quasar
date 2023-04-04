import * as React from 'react'

class ErrorBoundary extends React.Component {
  state: Readonly<any>
  declare props: Readonly<any>

  constructor(props: {} | Readonly<any>) {
    super(props)
    this.state = { hasError: false } as Readonly<any>
  }

  static getDerivedStateFromError(error: any) {
    // Update state so the next render will show the fallback UI.
    return { hasError: true }
  }

  componentDidCatch(error: any, errorInfo: any) {
    // You can also log the error to an error reporting service
    console.error(error, errorInfo)
  }

  render() {
    if (this.state.hasError) {
      // You can render any custom fallback UI
      return <text>Something went wrong.</text>
    }

    return this.props.children
  }
}

export default ErrorBoundary
