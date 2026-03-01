import { createServerFn } from '@tanstack/react-start'
import { getCookie, setCookie } from '@tanstack/react-start/server'
import { isThemeName, type ThemeName } from './themes-data'

const THEME_COOKIE_NAME = 'skoola-theme'

export const getThemeServer = createServerFn({ method: 'GET' }).handler(
  async () => {
    const themeValue = getCookie(THEME_COOKIE_NAME)
    return isThemeName(themeValue) ? themeValue : 'burgd'
  },
)

export const setThemeServer = createServerFn({ method: 'POST' })
  .inputValidator((theme: string) => theme)
  .handler(async ({ data: theme }) => {
    if (isThemeName(theme)) {
      setCookie(THEME_COOKIE_NAME, theme, {
        maxAge: 31536000, // 1 year
        path: '/',
        sameSite: 'lax',
      })
    }
    return { success: true }
  })
