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
import { HStack, Stack, Text } from '@/components/primitives'

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
        <HStack gap={3}>
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
          <Stack gap={0}>
            <Text className="font-medium text-sm">{staff.name}</Text>
            <Text size="xs" muted>
              {staff.employee_id}
            </Text>
          </Stack>
        </HStack>
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
            <DropdownMenuContent align="end" className="w-52">
              <DropdownMenuItem onClick={() => onEdit(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={PencilEdit01Icon} className="size-4" />
                  <span>Edit Profile</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onUploadPhoto(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Upload01Icon} className="size-4" />
                  <span>Upload Photo</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => onAssignClass(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={School01Icon} className="size-4" />
                  <span>Assign Class</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onAssignSubject(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={BookOpen01Icon} className="size-4" />
                  <span>Assign Subject</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => onViewWorkload(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Chart01Icon} className="size-4" />
                  <span>View Workload</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onManageAttendance(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon
                    icon={CalendarCheckIn01Icon}
                    className="size-4"
                  />
                  <span>Attendance</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onManageLeaves(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Calendar01Icon} className="size-4" />
                  <span>Leaves</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => onManagePermissions(staff)}>
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Layers01Icon} className="size-4" />
                  <span>Permissions</span>
                </HStack>
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem
                className="text-destructive focus:text-destructive"
                onClick={() => onDelete(staff.id)}
              >
                <HStack gap={2} p={0}>
                  <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                  <span>Delete</span>
                </HStack>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenuContent>
        </DropdownMenu>
      )
    },
  },
]
