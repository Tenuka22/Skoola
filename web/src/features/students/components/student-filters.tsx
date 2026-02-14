import { FilterIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useStudentsStore } from '../store'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'
import { DatePicker } from '@/components/ui/date-picker'

export function StudentFilters() {
  const {
    statusFilter,
    setStatusFilter,
    createdAfter,
    setCreatedAfter,
    createdBefore,
    setCreatedBefore,
  } = useStudentsStore()

  const hasFilters =
    statusFilter !== 'all' || !!createdAfter || !!createdBefore

  const clearFilters = () => {
    setStatusFilter('all')
    setCreatedAfter(null)
    setCreatedBefore(null)
  }

  return (
    <div className="mb-4 flex flex-wrap items-center gap-2 px-8">
      <DropdownMenu>
        <DropdownMenuTrigger render={
          <Button
            variant="outline"
            size="sm"
            className="h-8 rounded-lg border-none bg-muted/50 ring-1 ring-border"
          >
            <HugeiconsIcon icon={FilterIcon} className="mr-2 size-4" />
            {statusFilter === 'all' ? 'All Status' : statusFilter}
          </Button>
        } />
        <DropdownMenuContent align="start" className="w-56 rounded-xl p-2">
          <DropdownMenuLabel>Student Status</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuRadioGroup
            value={statusFilter}
            onValueChange={setStatusFilter}
          >
            <DropdownMenuRadioItem value="all">All Status</DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Active">Active</DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Suspended">
              Suspended
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Graduated">
              Graduated
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Transferred">
              Transferred
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Withdrawn">
              Withdrawn
            </DropdownMenuRadioItem>
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>

      <DatePicker
        label="Created after"
        value={createdAfter}
        onChange={setCreatedAfter}
      />
      <DatePicker
        label="Created before"
        value={createdBefore}
        onChange={setCreatedBefore}
      />

      {hasFilters && (
        <Button
          variant="ghost"
          size="sm"
          className="h-8 px-2 text-xs"
          onClick={clearFilters}
        >
          Clear Filters
        </Button>
      )}
    </div>
  )
}
