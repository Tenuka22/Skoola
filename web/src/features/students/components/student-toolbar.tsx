import {
  Add01Icon,
  DeleteIcon,
  Download01Icon,
  LayoutGridIcon,
  Search01Icon,
  TableIcon,
  Upload01Icon,
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

interface StudentToolbarProps {
  onExport: () => void
  onImport?: () => void
  selectedStudents: Set<string>
  onBulkDelete: () => void
  isBottomToolbar?: boolean
}

export function StudentToolbar({
  onExport,
  onImport,
  selectedStudents,
  onBulkDelete,
  isBottomToolbar = false,
}: StudentToolbarProps) {
  const { view, setView, search, setSearch, setIsCreateStudentOpen } =
    useStudentsStore()

  if (isBottomToolbar) {
    const selectedCount = selectedStudents.size
    if (selectedCount === 0) return null

    return (
      <div className="absolute bottom-4 left-1/2 z-10 -translate-x-1/2">
        <div className="flex items-center gap-4 rounded-lg border bg-background p-2 shadow-lg">
          <div className="text-sm font-medium">
            {selectedCount} student{selectedCount > 1 ? 's' : ''} selected
          </div>
          <Button
            variant="destructive"
            size="sm"
            className="gap-2"
            onClick={onBulkDelete}
          >
            <HugeiconsIcon icon={DeleteIcon} className="size-4" />
            Delete Selected
          </Button>
        </div>
      </div>
    )
  }

  return (
    <div className="mb-4 flex flex-col gap-4 px-8 sm:flex-row sm:items-center sm:justify-between">
      <Tabs value={view} onValueChange={(value: ViewMode) => setView(value)}>
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
          onClick={onImport}
        >
          <HugeiconsIcon icon={Upload01Icon} className="size-4" />
          Import
        </Button>

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
