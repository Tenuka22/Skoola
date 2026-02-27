import { Calendar01Icon, FilterIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { format } from 'date-fns'
import { useUsersStore } from '../store'
import { USER_AUTH_METHODS } from '../constants'
import { cn } from '@/lib/utils'

import { Button } from '@/components/ui/button'
import { Calendar } from '@/components/ui/calendar'
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { HStack } from '@/components/primitives'

export function UsersFilters() {
  const {
    statusFilter,
    setStatusFilter,
    authFilter,
    setAuthFilter,
    createdAfter,
    setCreatedAfter,
    createdBefore,
    setCreatedBefore,
  } = useUsersStore()

  return (
    <HStack p={0}>
      <Select
        value={statusFilter}
        onValueChange={(value) => setStatusFilter(value || 'all')}
      >
        <SelectTrigger className="w-fit min-w-32">
            <HStack gap={1} p={0}>

            <HugeiconsIcon icon={FilterIcon} className="size-4" />
            <SelectValue placeholder="Status" className="capitalize" />
</HStack>
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All Status</SelectItem>
          <SelectItem value="verified">Verified</SelectItem>
          <SelectItem value="unverified">Unverified</SelectItem>
        </SelectContent>
      </Select>

      <Select
        value={authFilter}
        onValueChange={(value) => setAuthFilter(value || 'all')}
      >
         <SelectTrigger className="w-fit min-w-32">
            <HStack gap={1} p={0}>

            <HugeiconsIcon icon={FilterIcon} className="size-4" />
            <SelectValue placeholder="Auth Method" className="capitalize" />
</HStack>
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">All Auth Methods</SelectItem>
          {USER_AUTH_METHODS.map((method) => (
            <SelectItem key={method} value={method}>
              {method === 'github'
                ? 'GitHub'
                : method === 'google'
                  ? 'Google'
                  : 'Password'}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      <Popover>
        <PopoverTrigger
          render={
            <Button
              variant="outline"
              className={cn(
                  !createdAfter && 'text-muted-foreground',
              )}
            >
              <HugeiconsIcon
                icon={Calendar01Icon}
                className="mr-2 h-4"
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
            initialFocus
          />
        </PopoverContent>
      </Popover>

      <Popover>
        <PopoverTrigger
          render={
            <Button
              variant="outline"
              className={cn(
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

      {(statusFilter !== 'all' ||
        authFilter !== 'all' ||
        createdAfter ||
        createdBefore) && (
        <Button
          variant="destructive" size="sm"
          onClick={() => {
            setStatusFilter('all')
            setAuthFilter('all')
            setCreatedAfter(null)
            setCreatedBefore(null)
          }}
        >
          Reset Filters
        </Button>
      )}
    </HStack>
  )
}
