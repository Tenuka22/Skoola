import { Calendar01Icon, FilterIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { format } from 'date-fns'
import { useStudentsStore } from '../store'
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

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { cn } from '@/lib/utils'
import { Calendar } from '@/components/ui/calendar'

export function StudentFilters() {
  const {
    statusFilter,
    setStatusFilter,
    createdAfter,
    setCreatedAfter,
    createdBefore,
    setCreatedBefore,
  } = useStudentsStore()

  const hasFilters = statusFilter !== 'all' || !!createdAfter || !!createdBefore

  const clearFilters = () => {
    setStatusFilter('all')
    setCreatedAfter(null)
    setCreatedBefore(null)
  }

  return (
    <div className="mb-4 flex flex-wrap items-center gap-2 px-8">
      <DropdownMenu>
        <DropdownMenuTrigger
          render={
            <Button variant="outline" size="sm">
              <HugeiconsIcon icon={FilterIcon} className="mr-2 size-4" />
              {statusFilter === 'all' ? 'All Status' : statusFilter}
            </Button>
          }
        />
        <DropdownMenuContent align="start" className="w-56">
          <DropdownMenuGroup>
            <DropdownMenuLabel>Student Status</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuRadioGroup
              value={statusFilter}
              onValueChange={setStatusFilter}
            >
              <DropdownMenuRadioItem value="all">
                All Status
              </DropdownMenuRadioItem>
              <DropdownMenuRadioItem value="Active">
                Active
              </DropdownMenuRadioItem>
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
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>

      <Popover>
        <PopoverTrigger
          render={
            <Button
              variant="outline"
              className={cn(
                'justify-start text-left font-normal',
                !createdAfter && 'text-muted-foreground',
              )}
            >
              <HugeiconsIcon
                icon={Calendar01Icon}
                className="mr-2 h-3.5 w-3.5"
              />
              {createdAfter
                ? format(new Date(createdAfter), 'PPP')
                : 'Created After'}
            </Button>
          }
        />
        <PopoverContent className="w-auto p-0" align="start">
          <Calendar
            mode="single"
            selected={createdAfter ? new Date(createdAfter) : undefined}
            onSelect={(date) =>
              setCreatedAfter(date ? format(date, 'yyyy-MM-dd') : null)
            }
          />
        </PopoverContent>
      </Popover>

      <Popover>
        <PopoverTrigger
          render={
            <Button
              variant="outline"
              className={cn(
                'justify-start text-left font-normal',
                !createdBefore && 'text-muted-foreground',
              )}
            >
              <HugeiconsIcon
                icon={Calendar01Icon}
                className="mr-2 h-3.5 w-3.5"
              />
              {createdBefore
                ? format(new Date(createdBefore), 'PPP')
                : 'Created Before'}
            </Button>
          }
        />
        <PopoverContent className="w-auto p-0" align="start">
          <Calendar
            mode="single"
            selected={createdBefore ? new Date(createdBefore) : undefined}
            onSelect={(date) =>
              setCreatedBefore(date ? format(date, 'yyyy-MM-dd') : null)
            }
          />
        </PopoverContent>
      </Popover>

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
