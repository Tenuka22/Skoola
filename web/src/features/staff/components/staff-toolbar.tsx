import {
  Add01Icon,
  Download01Icon,
  LayoutGridIcon,
  Search01Icon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useStaffStore } from '../store'
import type { StaffViewMode } from '../store'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'

interface StaffToolbarProps {
  onExport: () => void
}

export function StaffToolbar({ onExport }: StaffToolbarProps) {
  const { view, setView, search, setSearch, setIsCreateStaffOpen } =
    useStaffStore()

  return (
    <div className="mb-4 flex flex-col gap-4 px-8 sm:flex-row sm:items-center sm:justify-between">
      <Tabs
        value={view}
        onValueChange={(value: StaffViewMode) => setView(value)}
      >
        <TabsList>
          <TabsTrigger value="table" className="gap-2">
            <HugeiconsIcon icon={TableIcon} className="size-4" />
            Table
          </TabsTrigger>
          <TabsTrigger value="board" className="gap-2">
            <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
            Board
          </TabsTrigger>
        </TabsList>
      </Tabs>

      <div className="flex items-center gap-2 overflow-x-auto pb-2 sm:w-auto sm:pb-0">
        <div className="relative flex-1 sm:w-64">
          <InputGroup>
            <InputGroupInput
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Search staff..."
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
          onClick={onExport}
        >
          <HugeiconsIcon icon={Download01Icon} className="size-4" />
          Export
        </Button>

        <Button
          size="sm"
          className="gap-2"
          onClick={() => setIsCreateStaffOpen(true)}
        >
          <HugeiconsIcon icon={Add01Icon} className="size-4" />
          Add Staff
        </Button>
      </div>
    </div>
  )
}
