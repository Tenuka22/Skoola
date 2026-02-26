'use client'

import {
  BookOpen01Icon,
  Calendar01Icon,
  CalendarCheckIn01Icon,
  Chart01Icon,
  Delete02Icon,
  Layers01Icon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
  School01Icon,
  Upload01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { ColumnDef } from '@tanstack/react-table'
import type { StaffResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Checkbox } from '@/components/ui/checkbox'

interface GetStaffColumnsProps {
  onEdit: (staff: StaffResponse) => void
  onDelete: (id: string) => void
  onUploadPhoto: (staff: StaffResponse) => void
  onAssignClass: (staff: StaffResponse) => void
  onAssignSubject: (staff: StaffResponse) => void
  onViewWorkload: (staff: StaffResponse) => void
  onManageAttendance: (staff: StaffResponse) => void
  onManageLeaves: (staff: StaffResponse) => void
  onManagePermissions: (staff: StaffResponse) => void
}

export const getStaffColumns = ({
  onEdit,
  onDelete,
  onUploadPhoto,
  onAssignClass,
  onAssignSubject,
  onViewWorkload,
  onManageAttendance,
  onManageLeaves,
  onManagePermissions,
}: GetStaffColumnsProps): Array<ColumnDef<StaffResponse>> => [
  {
    id: 'select',
    header: ({ table }) => (
      <Checkbox
        checked={table.getIsAllPageRowsSelected()}
        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
        aria-label="Select all"
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
        aria-label="Select row"
      />
    ),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: 'name',
    header: 'Staff Member',
    cell: ({ row }) => {
      const staff = row.original
      return (
        <div className="flex items-center gap-3">
          <Avatar className="size-8">
            <AvatarImage
              src={
                staff.photo_url ||
                `https://api.dicebear.com/7.x/avataaars/svg?seed=${staff.email}`
              }
              alt={staff.name}
            />
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
      const type = row.original.staff_type
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
      const status = row.original.employment_status
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
          <DropdownMenuContent align="end" className="w-52">
            <DropdownMenuItem onClick={() => onEdit(staff)}>
              <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
              Edit Profile
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onUploadPhoto(staff)}>
              <HugeiconsIcon icon={Upload01Icon} className="mr-2 size-4" />
              Upload Photo
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => onAssignClass(staff)}>
              <HugeiconsIcon icon={School01Icon} className="mr-2 size-4" />
              Assign Class
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onAssignSubject(staff)}>
              <HugeiconsIcon icon={BookOpen01Icon} className="mr-2 size-4" />
              Assign Subject
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => onViewWorkload(staff)}>
              <HugeiconsIcon icon={Chart01Icon} className="mr-2 size-4" />
              View Workload
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onManageAttendance(staff)}>
              <HugeiconsIcon
                icon={CalendarCheckIn01Icon}
                className="mr-2 size-4"
              />
              Attendance
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onManageLeaves(staff)}>
              <HugeiconsIcon icon={Calendar01Icon} className="mr-2 size-4" />
              Leaves
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => onManagePermissions(staff)}>
              <HugeiconsIcon icon={Layers01Icon} className="mr-2 size-4" />
              Permissions
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              className="text-destructive focus:text-destructive"
              onClick={() => onDelete(staff.id)}
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
