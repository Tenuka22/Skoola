import {
  Add01Icon,
  Download01Icon,
  LayoutGridIcon,
  ListViewIcon,
  Search01Icon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useUsersStore } from '../store'
import { Input } from '@/components/ui/input'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
  InputGroupText,
  InputGroupTextarea,
} from '@/components/ui/input-group'

interface UsersToolbarProps {
  handleExportCSV: () => void
}

export function UsersToolbar({ handleExportCSV }: UsersToolbarProps) {
  const { view, setView, search, setSearch, setIsCreateUserOpen } =
    useUsersStore()

  return (
    <div className="mb-4 flex flex-col gap-4 px-8 sm:flex-row sm:items-center sm:justify-between">
      <Tabs value={view} onValueChange={(value: any) => setView(value)}>
        <TabsList>
          <TabsTrigger value="table" className="gap-2">
            <HugeiconsIcon icon={TableIcon} className="size-4" />
            Table
          </TabsTrigger>
          <TabsTrigger value="board" className="gap-2">
            <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
            Board
          </TabsTrigger>
          <TabsTrigger value="list" className="gap-2">
            <HugeiconsIcon icon={ListViewIcon} className="size-4" />
            List
          </TabsTrigger>
        </TabsList>
      </Tabs>

      <div className="flex items-center gap-2 overflow-x-auto pb-2 sm:w-auto sm:pb-0">
        <div className="relative flex-1 sm:w-64">
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
        </div>

        <Button
          variant="outline"
          size="sm"
          className="gap-2"
          onClick={handleExportCSV}
        >
          Export CSV
          <HugeiconsIcon icon={Download01Icon} className="size-4" />
        </Button>

        <Button
          size="sm"
          className="gap-2"
          onClick={() => setIsCreateUserOpen(true)}
        >
          Add User
          <HugeiconsIcon icon={Add01Icon} className="size-4" />
        </Button>
      </div>
    </div>
  )
}
