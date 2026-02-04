import {
  AppTheme,
  getStoredUserTheme,
  getSystemTheme,
  handleThemeChange,
  setStoredTheme,
  setupPreferredListener,
  themeScript,
  UserTheme,
} from '@/lib/theme'
import { ScriptOnce } from '@tanstack/react-router'
import { createContext, ReactNode, use, useEffect, useState } from 'react'

type ThemeContextProps = {
  userTheme: UserTheme
  appTheme: AppTheme
  setTheme: (theme: UserTheme) => void
}

const ThemeContext = createContext<ThemeContextProps | undefined>(undefined)

type ThemeProviderProps = {
  children: ReactNode
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const [userTheme, setUserTheme] = useState<UserTheme>(getStoredUserTheme)

  useEffect(() => {
    if (userTheme !== 'system') return
    return setupPreferredListener(handleThemeChange)
  }, [userTheme])

  const appTheme = userTheme === 'system' ? getSystemTheme() : userTheme

  const setTheme = (newUserTheme: UserTheme) => {
    setUserTheme(newUserTheme)
    setStoredTheme(newUserTheme)
    handleThemeChange(newUserTheme)
  }

  return (
    <ThemeContext value={{ userTheme, appTheme, setTheme }}>
      <ScriptOnce children={themeScript} />
      {children}
    </ThemeContext>
  )
}

export const useTheme = () => {
  const context = use(ThemeContext)
  if (!context) {
    throw new Error('useTheme must be used within a ThemeProvider')
  }
  return context
}