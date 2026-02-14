import { FilterIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useStaffStore } from '../store'
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

export function StaffFilters() {
  const {
    staffTypeFilter,
    setStaffTypeFilter,
    employmentStatusFilter,
    setEmploymentStatusFilter,
  } = useStaffStore()

  return (
    <div className="mb-4 flex flex-wrap items-center gap-2 px-8">
      <DropdownMenu>
        <DropdownMenuTrigger
          render={
            <Button
              variant="outline"
              size="sm"
              className="h-8 rounded-lg border-none bg-muted/50 ring-1 ring-border"
            >
              <HugeiconsIcon icon={FilterIcon} className="mr-2 size-4" />
              {staffTypeFilter === 'all' ? 'All Roles' : staffTypeFilter}
            </Button>
          }
        />
        <DropdownMenuContent align="start" className="w-56 rounded-xl p-2">
          <DropdownMenuLabel>Staff Type</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuRadioGroup
            value={staffTypeFilter}
            onValueChange={setStaffTypeFilter}
          >
            <DropdownMenuRadioItem value="all">All Roles</DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Teaching">
              Teaching
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="NonTeaching">
              Non-Teaching
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Administrative">
              Administrative
            </DropdownMenuRadioItem>
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>

      <DropdownMenu>
        <DropdownMenuTrigger
          render={
            <Button
              variant="outline"
              size="sm"
              className="h-8 rounded-lg border-none bg-muted/50 ring-1 ring-border"
            >
              <HugeiconsIcon icon={FilterIcon} className="mr-2 size-4" />
              {employmentStatusFilter === 'all'
                ? 'All Status'
                : employmentStatusFilter}
            </Button>
          }
        />
        <DropdownMenuContent align="start" className="w-56 rounded-xl p-2">
          <DropdownMenuLabel>Employment Status</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuRadioGroup
            value={employmentStatusFilter}
            onValueChange={setEmploymentStatusFilter}
          >
            <DropdownMenuRadioItem value="all">
              All Status
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Permanent">
              Permanent
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Contract">
              Contract
            </DropdownMenuRadioItem>
            <DropdownMenuRadioItem value="Temporary">
              Temporary
            </DropdownMenuRadioItem>
          </DropdownMenuRadioGroup>
        </DropdownMenuContent>
      </DropdownMenu>

      {(staffTypeFilter !== 'all' || employmentStatusFilter !== 'all') && (
        <Button
          variant="ghost"
          size="sm"
          className="h-8 px-2 text-xs"
          onClick={() => {
            setStaffTypeFilter('all')
            setEmploymentStatusFilter('all')
          }}
        >
          Clear Filters
        </Button>
      )}
    </div>
  )
}
