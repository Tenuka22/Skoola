import {
  Add01Icon,
  Download01Icon,
  LayoutGridIcon,
  Search01Icon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useStudentsStore } from '../store'
import type { ViewMode } from '../store'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Button } from '@/components/ui/button'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'

interface StudentsToolbarProps {
  onExport: () => void
}

export function StudentsToolbar({ onExport }: StudentsToolbarProps) {
  const { view, setView, search, setSearch, setIsCreateStudentOpen } =
    useStudentsStore()

  return (
    <div className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <Tabs value={view} onValueChange={(value: ViewMode) => setView(value)}>
        <TabsList className="bg-muted/50 p-1">
          <TabsTrigger value="table" className="gap-2">
            <HugeiconsIcon icon={TableIcon} className="size-4" />
            Table
          </TabsTrigger>
          <TabsTrigger value="grid" className="gap-2">
            <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
            Grid
          </TabsTrigger>
        </TabsList>
      </Tabs>

      <div className="flex items-center gap-2 overflow-x-auto pb-2 sm:w-auto sm:pb-0">
        <div className="relative flex-1 sm:w-64">
          <InputGroup>
            <InputGroupInput
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Search students..."
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
          onClick={() => setIsCreateStudentOpen(true)}
        >
          <HugeiconsIcon icon={Add01Icon} className="size-4" />
          Add Student
        </Button>
      </div>
    </div>
  )
}
