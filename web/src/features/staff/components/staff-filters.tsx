import { FilterIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useStaffStore } from '../store'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'
import { zEmploymentStatus, zStaffType } from '@/lib/api/zod.gen'

export function StaffFilters() {
  const {
    staffTypeFilter,
    setStaffTypeFilter,
    employmentStatusFilter,
    setEmploymentStatusFilter,
  } = useStaffStore()

  const staffTypes = zStaffType.options
  const employmentStatuses = zEmploymentStatus.options

  return (
    <div className="mb-4 flex flex-wrap items-center gap-2 px-8">
      <DropdownMenu>
        <DropdownMenuTrigger
          render={
            <Button variant="outline" size="sm">
              <HugeiconsIcon icon={FilterIcon} className="mr-2 size-4" />
              {staffTypeFilter === 'all' ? 'All Roles' : staffTypeFilter}
            </Button>
          }
        ></DropdownMenuTrigger>

        <DropdownMenuContent align="start" className="w-56">
          <DropdownMenuGroup>
            <DropdownMenuLabel>Staff Type</DropdownMenuLabel>
            <DropdownMenuSeparator />

            <DropdownMenuRadioGroup
              value={staffTypeFilter}
              onValueChange={setStaffTypeFilter}
            >
              <DropdownMenuRadioItem value="all">
                All Roles
              </DropdownMenuRadioItem>
              {staffTypes.map((type) => (
                <DropdownMenuRadioItem key={type} value={type}>
                  {type === 'NonTeaching' ? 'Non-Teaching' : type}
                </DropdownMenuRadioItem>
              ))}
            </DropdownMenuRadioGroup>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>

      <DropdownMenu>
        <DropdownMenuTrigger
          render={
            <Button variant="outline" size="sm">
              <HugeiconsIcon icon={FilterIcon} className="mr-2 size-4" />
              {employmentStatusFilter === 'all'
                ? 'All Status'
                : employmentStatusFilter}
            </Button>
          }
        ></DropdownMenuTrigger>

        <DropdownMenuContent align="start" className="w-56">
          <DropdownMenuGroup>
            <DropdownMenuLabel>Employment Status</DropdownMenuLabel>
            <DropdownMenuSeparator />

            <DropdownMenuRadioGroup
              value={employmentStatusFilter}
              onValueChange={setEmploymentStatusFilter}
            >
              <DropdownMenuRadioItem value="all">
                All Status
              </DropdownMenuRadioItem>
              {employmentStatuses.map((status) => (
                <DropdownMenuRadioItem key={status} value={status}>
                  {status}
                </DropdownMenuRadioItem>
              ))}
            </DropdownMenuRadioGroup>
          </DropdownMenuGroup>
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
