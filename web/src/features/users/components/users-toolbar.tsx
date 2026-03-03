import * as React from 'react'
import {
  Add01Icon,
  LayoutGridIcon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useUsersSearchParams } from '../search-params'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'

interface UsersToolbarProps {
  setIsCreateUserOpen: (open: boolean) => void
}

export function UsersToolbar({ setIsCreateUserOpen }: UsersToolbarProps) {
  const { view, setView } = useUsersSearchParams()

  // Keyboard shortcuts
  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Alt/Option + N to Add User
      if (e.altKey && e.code === 'KeyN') {
        e.preventDefault()
        setIsCreateUserOpen(true)
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [setIsCreateUserOpen])

  return (
    <HStack align="center" justify="between" className="mb-6">
      <Tabs
        value={view ?? 'table'}
        onValueChange={(value: string) => setView(value)}
      >
        <TabsList className="bg-muted/50 p-1">
          <TabsTrigger value="table">
            <HStack gap={1} p={0}>
              <HugeiconsIcon icon={TableIcon} className="size-4" />
              Table
            </HStack>
          </TabsTrigger>
          <TabsTrigger value="grid">
            <HStack gap={1} p={0}>
              <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
              Grid
            </HStack>
          </TabsTrigger>
        </TabsList>
      </Tabs>

      <Button size="sm" onClick={() => setIsCreateUserOpen(true)}>
        <HStack gap={1} p={0} align="center">
          Add User
          <HugeiconsIcon icon={Add01Icon} className="size-4 mr-1" />
          <kbd className="pointer-events-none hidden sm:inline-flex shrink-0 select-none items-center gap-0.5 rounded border border-primary-foreground/20 bg-primary-foreground/10 px-1.5 py-0.5 text-[10px] font-medium text-primary-foreground">
            <span className="text-xs mr-0.5">⌥</span>N
          </kbd>
        </HStack>
      </Button>
    </HStack>
  )
}
