import { useTheme } from '../providers/theme-provider'
import { Button } from '../ui/button'
import type { UserTheme } from '@/lib/theme'

const themeConfig: Record<UserTheme, { icon: string; label: string }> = {
  light: { icon: '☀️', label: 'Light' },
  dark: { icon: '🌙', label: 'Dark' },
  system: { icon: '💻', label: 'System' },
}

export const ThemeToggle = () => {
  const { userTheme, setTheme } = useTheme()

  const getNextTheme = () => {
    const themes: Array<UserTheme> = ['light', 'dark', 'system']
    const currentIndex = themes.indexOf(userTheme)
    const nextIndex = (currentIndex + 1) % themes.length
    return themes[nextIndex]
  }

  return (
    <Button onClick={() => setTheme(getNextTheme())} className="w-28">
      {userTheme === 'light' && (
        <>
          {themeConfig.light.label}
          <span className="ml-1">{themeConfig.light.icon}</span>
        </>
      )}
      {userTheme === 'dark' && (
        <>
          {themeConfig.dark.label}
          <span className="ml-1">{themeConfig.dark.icon}</span>
        </>
      )}
      {userTheme === 'system' && (
        <>
          {themeConfig.system.label}
          <span className="ml-1">{themeConfig.system.icon}</span>
        </>
      )}
    </Button>
  )
}
