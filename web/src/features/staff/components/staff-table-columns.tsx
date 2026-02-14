import type { ColumnDef } from '@tanstack/react-table'
import {
  MoreHorizontalIcon,
  PencilEdit01Icon,
  Delete02Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { StaffResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'

interface GetStaffColumnsProps {
  onEdit: (staff: StaffResponse) => void
  onDelete: (staff: StaffResponse) => void
}

export const getStaffColumns = ({
  onEdit,
  onDelete,
}: GetStaffColumnsProps): Array<ColumnDef<StaffResponse>> => [
  {
    accessorKey: 'name',
    header: 'Staff Member',
    cell: ({ row }) => {
      const staff = row.original
      return (
        <div className="flex items-center gap-3">
          <Avatar className="size-8">
            <AvatarImage src={staff.photo_url || undefined} alt={staff.name} />
            <AvatarFallback>
              {staff.name
                .split(' ')
                .map((n) => n[0])
                .join('')
                .toUpperCase()}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <span className="font-medium">{staff.name}</span>
            <span className="text-xs text-muted-foreground">
              {staff.employee_id}
            </span>
          </div>
        </div>
      )
    },
  },
  {
    accessorKey: 'email',
    header: 'Email',
  },
  {
    accessorKey: 'staff_type',
    header: 'Role',
    cell: ({ row }) => {
      const type = row.getValue('staff_type') as string
      return (
        <Badge variant="outline" className="font-normal">
          {type}
        </Badge>
      )
    },
  },
  {
    accessorKey: 'employment_status',
    header: 'Status',
    cell: ({ row }) => {
      const status = row.getValue('employment_status') as string
      return (
        <Badge
          variant="secondary"
          className={
            status === 'Permanent'
              ? 'bg-green-500/10 text-green-600 border-green-500/20'
              : 'bg-blue-500/10 text-blue-600 border-blue-500/20'
          }
        >
          {status}
        </Badge>
      )
    },
  },
  {
    accessorKey: 'phone',
    header: 'Phone',
  },
  {
    id: 'actions',
    cell: ({ row }) => {
      const staff = row.original
      return (
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button variant="ghost" size="icon" className="size-8">
                <HugeiconsIcon icon={MoreHorizontalIcon} className="size-4" />
              </Button>
            }
          />
          <DropdownMenuContent align="end" className="w-40">
            <DropdownMenuItem onClick={() => onEdit(staff)}>
              <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuItem
              className="text-destructive focus:text-destructive"
              onClick={() => onDelete(staff)}
            >
              <HugeiconsIcon icon={Delete02Icon} className="mr-2 size-4" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )
    },
  },
]
