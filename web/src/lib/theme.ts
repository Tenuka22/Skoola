import { z } from 'zod'

export const themeStorageKey = 'ui-theme'

export const UserThemeSchema = z
  .enum(['light', 'dark', 'system'])
  .catch('system')
export const AppThemeSchema = z.enum(['light', 'dark']).catch('light')

export type UserTheme = z.infer<typeof UserThemeSchema>
export type AppTheme = z.infer<typeof AppThemeSchema>

export function getSystemTheme(): AppTheme {
  if (typeof window === 'undefined') return 'light'
  return window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light'
}

export function setupPreferredListener(
  onThemeChange: (userTheme: UserTheme) => void,
) {
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  const handler = () => onThemeChange('system')
  mediaQuery.addEventListener('change', handler)
  return () => mediaQuery.removeEventListener('change', handler)
}

export function handleThemeChange(userTheme: UserTheme) {
  const root = document.documentElement
  root.classList.remove('light', 'dark', 'system')
  const newTheme = userTheme === 'system' ? getSystemTheme() : userTheme
  root.classList.add(newTheme)
  if (userTheme === 'system') {
    root.classList.add('system')
  }
}

export const themeScript: string = (function () {
  function themeFn() {
    try {
      const storedTheme = localStorage.getItem('ui-theme') || 'system'
      const validTheme = ['light', 'dark', 'system'].includes(storedTheme)
        ? storedTheme
        : 'system'
      if (validTheme === 'system') {
        const systemTheme = window.matchMedia('(prefers-color-scheme: dark)')
          .matches
          ? 'dark'
          : 'light'
        document.documentElement.classList.add(systemTheme, 'system')
      } else {
        document.documentElement.classList.add(validTheme)
      }
    } catch {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)')
        .matches
        ? 'dark'
        : 'light'
      document.documentElement.classList.add(systemTheme, 'system')
    }
  }
  return `(${themeFn.toString()})();`
})()

export function getStoredUserTheme(): UserTheme {
  if (typeof window === 'undefined') return 'system'
  try {
    const stored = localStorage.getItem(themeStorageKey)
    return UserThemeSchema.parse(stored)
  } catch {
    return 'system'
  }
}

export function setStoredTheme(theme: UserTheme): void {
  if (typeof window === 'undefined') return
  try {
    localStorage.setItem(themeStorageKey, theme)
  } catch {}
}
