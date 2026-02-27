import {
  Add01Icon,
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
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'
import { Box, HStack } from '@/components/primitives'

interface UsersToolbarProps {
  handleExportCSV: () => void
}

export function UsersToolbar({ handleExportCSV }: UsersToolbarProps) {
  const { view, setView, search, setSearch, setIsCreateUserOpen } =
    useUsersStore()

  return (
    <HStack align="center" justify="between">
      <Tabs value={view} onValueChange={(value: ViewMode) => setView(value)}>
        <TabsList>
          <TabsTrigger value="table">
            <HStack gap={1} p={0}>
              <HugeiconsIcon icon={TableIcon} className="size-4" />
              Table
            </HStack>
          </TabsTrigger>
          <TabsTrigger value="board">
            <HStack gap={1} p={0}>
              <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
              Board
            </HStack>
          </TabsTrigger>
        </TabsList>
      </Tabs>

      <HStack align="center" gap={2}>
        <Box className="relative flex-1 sm:w-64">
          <InputGroup>
            <InputGroupInput
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Search..."
            />
            <InputGroupAddon>
              <HugeiconsIcon icon={Search01Icon} />
            </InputGroupAddon>
          </InputGroup>
        </Box>

        <Button variant="outline" size="sm" onClick={handleExportCSV}>
          <HStack gap={1} p={0}>
            Export CSV
            <HugeiconsIcon icon={Download01Icon} className="size-4" />
          </HStack>
        </Button>

        <Button size="sm" onClick={() => setIsCreateUserOpen(true)}>
          <HStack gap={1} p={0}>
            Add User
            <HugeiconsIcon icon={Add01Icon} className="size-4" />
          </HStack>
        </Button>
      </HStack>
    </HStack>
  )
}
