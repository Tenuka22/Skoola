import * as React from 'react'
import {
  Add01Icon,
  Cancel01Icon,
  Download01Icon,
  LayoutGridIcon,
  Search01Icon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useUsersStore } from '../store'
import type { ViewMode } from '../store'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import { HStack } from '@/components/primitives'
import { cn } from '@/lib/utils'

interface UsersToolbarProps {
  handleExportCSV: () => void
}

export function UsersToolbar({ handleExportCSV }: UsersToolbarProps) {
  const { view, setView, search, setSearch, setIsCreateUserOpen } =
    useUsersStore()
  const inputRef = React.useRef<HTMLInputElement>(null)

  // Keyboard shortcuts
  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Cmd/Ctrl + K to focus search
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault()
        inputRef.current?.focus()
      }
      // Alt/Option + N to Add User
      if (e.altKey && e.code === 'KeyN') {
        e.preventDefault()
        setIsCreateUserOpen(true)
      }
      // Escape to blur and clear search
      if (e.key === 'Escape' && document.activeElement === inputRef.current) {
        inputRef.current?.blur()
      }
    }
    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [setIsCreateUserOpen])

  return (
    <HStack align="center" justify="between" className="mb-6">
      <Tabs value={view} onValueChange={(value: ViewMode) => setView(value)}>
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

      <HStack align="center" gap={2}>
        {/* Search Bar */}
        <div className="relative group">
          <div
            className={cn(
              'flex items-center gap-2 h-9 w-72 rounded-lg border border-border/60 bg-background px-3 transition-all duration-200',
              'focus-within:border-ring focus-within:ring-2 focus-within:ring-ring/20 focus-within:w-80',
              'hover:border-border',
            )}
          >
            <HugeiconsIcon
              icon={Search01Icon}
              className="size-4 text-muted-foreground shrink-0"
            />
            <input
              ref={inputRef}
              type="text"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Search users..."
              className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground/60"
            />
            {search ? (
              <button
                type="button"
                onClick={() => {
                  setSearch('')
                  inputRef.current?.focus()
                }}
                className="shrink-0 rounded-sm p-0.5 text-muted-foreground hover:text-foreground transition-colors"
              >
                <HugeiconsIcon icon={Cancel01Icon} size={14} strokeWidth={2} />
              </button>
            ) : (
              <kbd className="pointer-events-none hidden sm:inline-flex shrink-0 select-none items-center gap-0.5 rounded border border-border/60 bg-muted/50 px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground">
                <span className="text-xs mr-0.5">⌘</span>K
              </kbd>
            )}
          </div>
        </div>

        <Button variant="outline" size="sm" onClick={handleExportCSV}>
          <HStack gap={1} p={0}>
            Export CSV
            <HugeiconsIcon icon={Download01Icon} className="size-4" />
          </HStack>
        </Button>

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
    </HStack>
  )
}
