import { Monitor, Moon, Palette, Sun } from 'lucide-react'
import { isThemeName, themes } from '../../lib/themes-data'
import { useTheme } from '../providers/theme-provider'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '../ui/dropdown-menu'
import { HStack, Text } from '../primitives'
import { Button } from '../ui/button'
import { cn } from '@/lib/utils'

export const ThemeToggle = () => {
  const { theme, setTheme, mode, setMode } = useTheme()

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <Button variant="outline" size="icon-lg" className="rounded-full">
            <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
            <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
            <span className="sr-only">Toggle theme</span>
          </Button>
        }
      />
      <DropdownMenuContent align="end" className="w-56">
        <DropdownMenuGroup>
          <DropdownMenuLabel>
            <Text
              size="xs"
              muted
              className="font-semibold uppercase tracking-wider"
            >
              Appearance
            </Text>
          </DropdownMenuLabel>
          <DropdownMenuItem
            onClick={() => setMode('light')}
            className={cn(
              mode === 'light' && 'bg-accent text-accent-foreground',
            )}
          >
            <Sun className="mr-2 h-4 w-4" />
            Light
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => setMode('dark')}
            className={cn(
              mode === 'dark' && 'bg-accent text-accent-foreground',
            )}
          >
            <Moon className="mr-2 h-4 w-4" />
            Dark
          </DropdownMenuItem>
          <DropdownMenuItem
            onClick={() => setMode('system')}
            className={cn(
              mode === 'system' && 'bg-accent text-accent-foreground',
            )}
          >
            <Monitor className="mr-2 h-4 w-4" />
            System
          </DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuLabel>
            <Text
              size="xs"
              muted
              className="font-semibold uppercase tracking-wider"
            >
              Theme
            </Text>
          </DropdownMenuLabel>
          {Object.keys(themes).map((t) => {
            if (!isThemeName(t)) return null
            return (
              <DropdownMenuItem
                key={t}
                onClick={() => setTheme(t)}
                className={cn(
                  'capitalize',
                  theme === t && 'bg-accent text-accent-foreground',
                )}
              >
                <HStack gap={2} p={0}>
                  <Palette className="h-4 w-4" />
                  {t}
                </HStack>
              </DropdownMenuItem>
            )
          })}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
