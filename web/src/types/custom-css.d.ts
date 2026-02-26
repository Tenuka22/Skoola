import 'react'

declare module 'react' {
  interface CSSProperties {
    '--ratio'?: string | number
    '--color-bg'?: string
    '--color-border'?: string
    '--sidebar-width'?: string
    '--sidebar-width-icon'?: string
    '--skeleton-width'?: string
  }
}
