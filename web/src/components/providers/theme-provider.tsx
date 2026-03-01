import {
  ThemeProvider as NextThemesProvider,
  useTheme as useNextTheme,
} from 'next-themes'
import { createContext, useContext, useEffect, useState } from 'react'
import { isThemeName, themes } from '../../lib/themes-data'
import type { ThemeName } from '../../lib/themes-data'
import type { ReactNode } from 'react'
import { setThemeServer } from '@/lib/theme-server'

type ThemeProviderProps = {
  children: ReactNode
  initialTheme?: ThemeName
}

type ThemeContextType = {
  theme: ThemeName
  setTheme: (theme: ThemeName) => void
  mode: string | undefined
  setMode: (mode: string) => void
  resolvedMode: string | undefined
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined)

export function ThemeProvider({ children, initialTheme }: ThemeProviderProps) {
  const [theme, setThemeState] = useState<ThemeName>(() => {
    if (initialTheme) return initialTheme
    if (typeof window !== 'undefined') {
      const saved = document.cookie
        .split('; ')
        .find((row) => row.startsWith('skoola-theme='))
        ?.split('=')[1]
      if (isThemeName(saved)) return saved
    }
    return 'burgd'
  })

  useEffect(() => {
    // Set cookie via server function for persistence
    setThemeServer({ data: theme })
    document.documentElement.setAttribute('data-theme', theme)

    // Inject styles
    let styleTag = document.getElementById('skoola-theme-styles')
    if (!styleTag) {
      styleTag = document.createElement('style')
      styleTag.id = 'skoola-theme-styles'
      document.head.appendChild(styleTag)
    }

    const themeData = themes[theme]
    if (themeData) {
      styleTag.innerHTML = `
        :root {
          ${themeData.root}
        }
        .dark {
          ${themeData.dark}
        }
      `
    }
  }, [theme])

  return (
    <NextThemesProvider
      attribute="class"
      defaultTheme="system"
      enableSystem
      disableTransitionOnChange
    >
      <ThemeWrapper theme={theme} setTheme={setThemeState}>
        {children}
      </ThemeWrapper>
    </NextThemesProvider>
  )
}

function ThemeWrapper({
  children,
  theme,
  setTheme,
}: {
  children: ReactNode
  theme: ThemeName
  setTheme: (theme: ThemeName) => void
}) {
  const { theme: mode, setTheme: setMode, resolvedTheme } = useNextTheme()

  return (
    <ThemeContext.Provider
      value={{
        theme,
        setTheme,
        mode,
        setMode,
        resolvedMode: resolvedTheme,
      }}
    >
      {children}
    </ThemeContext.Provider>
  )
}

export const useTheme = () => {
  const context = useContext(ThemeContext)
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider')
  }
  return context
}
