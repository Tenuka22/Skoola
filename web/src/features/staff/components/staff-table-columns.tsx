import {
  Copy01Icon,
  Delete02Icon,
  MoreHorizontalCircle01Icon,
  PencilEdit01Icon,
  User02Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { toast } from 'sonner'
import { format } from 'date-fns'
import type { Staff } from '../types'
import type { DataTableColumnDef } from '@/components/data-table'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { cn } from '@/lib/utils'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import {
  ContextMenuItem,
  ContextMenuSeparator,
} from '@/components/ui/context-menu'
import { HStack, Stack, Text } from '@/components/primitives'
import { DataTableColumnHeader } from '@/components/data-table'

interface GetColumnsProps {
  staff?: Array<Staff>
  onEdit: (staff: Staff) => void
  onDelete: (id: string) => void
  onViewProfile?: (staff: Staff) => void
}

export function StaffContextMenuItems({
  staff,
  onEdit,
  onDelete,
  onViewProfile,
}: Omit<GetColumnsProps, 'staff'> & { staff: Staff }) {
  return (
    <>
      <div className="px-2 py-1.5 text-sm font-medium text-muted-foreground truncate max-w-xs">
        {staff.name}
      </div>
      <ContextMenuSeparator />

      <ContextMenuItem
        onClick={() => {
          navigator.clipboard.writeText(staff.id)
          toast.success('Staff ID copied to clipboard')
        }}
      >
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={Copy01Icon} className="h-4 w-4" />
          <span>Copy ID</span>
        </HStack>
      </ContextMenuItem>

      <ContextMenuItem
        onClick={() => {
          navigator.clipboard.writeText(staff.employee_id)
          toast.success('Employee ID copied to clipboard')
        }}
      >
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={Copy01Icon} className="h-4 w-4" />
          <span>Copy Employee ID</span>
        </HStack>
      </ContextMenuItem>

      <ContextMenuSeparator />

      <ContextMenuItem onClick={() => onEdit(staff)}>
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={PencilEdit01Icon} className="h-4 w-4" />
          <span>Edit</span>
        </HStack>
      </ContextMenuItem>

      {onViewProfile && (
        <ContextMenuItem onClick={() => onViewProfile(staff)}>
          <HStack gap={2} p={0}>
            <HugeiconsIcon icon={User02Icon} className="h-4 w-4" />
            <span>View Profile</span>
          </HStack>
        </ContextMenuItem>
      )}

      <ContextMenuSeparator />

      <ContextMenuItem
        onClick={() => onDelete(staff.id)}
        className="text-destructive focus:text-destructive"
      >
        <HStack gap={2} p={0}>
          <HugeiconsIcon icon={Delete02Icon} className="h-4 w-4" />
          <span>Delete</span>
        </HStack>
      </ContextMenuItem>
    </>
  )
}

export function getStaffColumns({
  onEdit,
  onDelete,
  onViewProfile,
}: GetColumnsProps): Array<DataTableColumnDef<Staff>> {
  return [
    {
      accessorKey: 'name',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Staff Member" />
      ),
      cell: ({ row }) => {
        const staff = row.original
        return (
          <HStack gap={3} align="center">
            <Avatar className="h-9 w-9">
              <AvatarImage src={staff.photo_url ?? undefined} alt={staff.name} />
              <AvatarFallback>
                {staff.name
                  .split(' ')
                  .map((n) => n[0])
                  .join('')
                  .toUpperCase()}
              </AvatarFallback>
            </Avatar>
            <Stack gap={1}>
              <Text size="sm" className="font-medium">
                {staff.name}
              </Text>
              <Text size="xs" muted>
                {staff.email || 'No email'}
              </Text>
            </Stack>
          </HStack>
        )
      },
      enableSorting: true,
    },
    {
      accessorKey: 'employee_id',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Employee ID" />
      ),
      cell: ({ row }) => (
        <Text size="sm">{row.getValue('employee_id')}</Text>
      ),
      enableSorting: true,
    },
    {
      accessorKey: 'staff_type',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Staff Type" />
      ),
      cell: ({ row }) => {
        const staffType = String(row.getValue('staff_type'))
        return (
          <Badge
            variant="secondary"
            className={cn(
              'text-[10px] px-1.5 py-0',
              staffType === 'Teaching' && 'bg-blue-500/10 text-blue-500',
              staffType === 'NonTeaching' &&
                'bg-purple-500/10 text-purple-500',
              staffType === 'Administrative' &&
                'bg-orange-500/10 text-orange-500',
            )}
          >
            {staffType}
          </Badge>
        )
      },
      filterFn: (row, id, value: Array<unknown>) => {
        const cellValue = row.getValue(id)
        return value.includes(cellValue)
      },
      enableSorting: true,
    },
    {
      accessorKey: 'employment_status',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Status" />
      ),
      cell: ({ row }) => {
        const status = row.getValue('employment_status')
        if (!status) return <Text muted size="sm">No status</Text>
        return (
          <Badge
            variant="outline"
            className={cn(
              'text-[10px] px-1.5 py-0',
              status === 'Active' && 'bg-green-500/10 text-green-500',
              status === 'Inactive' && 'bg-red-500/10 text-red-500',
              status === 'OnLeave' && 'bg-yellow-500/10 text-yellow-500',
              status === 'Terminated' && 'bg-gray-500/10 text-gray-500',
              status === 'Retired' && 'bg-blue-500/10 text-blue-500',
            )}
          >
            {String(status)}
          </Badge>
        )
      },
      filterFn: (row, id, value: Array<unknown>) => {
        const cellValue = row.getValue(id)
        return value.includes(cellValue)
      },
      enableSorting: true,
    },
    {
      accessorKey: 'phone',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Phone" />
      ),
      cell: ({ row }) => (
        <Text size="sm" muted>
          {row.getValue('phone') || '—'}
        </Text>
      ),
      enableSorting: true,
    },
    {
      accessorKey: 'dob',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Date of Birth" />
      ),
      cell: ({ row }) => {
        const dob = String(row.getValue('dob'))
        return (
          <Text size="sm" muted>
            {format(new Date(dob), 'MMM dd, yyyy')}
          </Text>
        )
      },
      enableSorting: true,
    },
    {
      id: 'actions',
      cell: ({ row }) => {
        const staff = row.original
        return (
          <DropdownMenu>
            <DropdownMenuTrigger render={
              <Button variant="ghost" size="icon" className="h-8 w-8">
                <HugeiconsIcon
                  icon={MoreHorizontalCircle01Icon}
                  className="h-4 w-4"
                />
              </Button>
            }></DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <StaffContextMenuItems
                staff={staff}
                onEdit={onEdit}
                onDelete={onDelete}
                onViewProfile={onViewProfile}
              />
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
    },
  ]
}
