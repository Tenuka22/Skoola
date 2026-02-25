'use client'

import {
  ArrowDown01Icon,
  ArrowUp01Icon,
  CalendarCheckIn01Icon,
  Chart01Icon,
  Delete02Icon,
  Menu01Icon,
  PencilEdit01Icon,
  School01Icon,
  Upload01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import type { ColumnDef } from '@tanstack/react-table'
import type { StudentResponse } from '@/lib/api/types.gen'
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

interface GetStudentColumnsProps {
  onEdit: (student: StudentResponse) => void
  onDelete: (id: string) => void
  onUploadPhoto: (student: StudentResponse) => void
  onAssignClass: (student: StudentResponse) => void
  onManageGuardians: (student: StudentResponse) => void
  onManageAttendance: (student: StudentResponse) => void
  onManageMarks: (student: StudentResponse) => void
}

export const getStudentColumns = ({
  onEdit,
  onDelete,
  onUploadPhoto,
  onAssignClass,
  onManageGuardians,
  onManageAttendance,
  onManageMarks,
}: GetStudentColumnsProps): Array<ColumnDef<StudentResponse>> => [
  {
    id: 'select',
    header: ({ table }) => (
      <Checkbox
        checked={table.getIsAllPageRowsSelected()}
        onCheckedChange={(value) => {
          table.toggleAllPageRowsSelected(!!value)
        }}
      />
    ),
    cell: ({ row }) => (
      <Checkbox
        checked={row.getIsSelected()}
        onCheckedChange={(value) => row.toggleSelected(!!value)}
      />
    ),
    enableSorting: false,
    enableHiding: false,
    size: 16,
  },
  {
    accessorKey: 'name_english',
    header: ({ column }) => (
      <Button
        variant="ghost"
        onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
      >
        Student Name
        {column.getIsSorted() === 'asc' ? (
          <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
        ) : column.getIsSorted() === 'desc' ? (
          <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
        ) : null}
      </Button>
    ),
    cell: ({ row }) => {
      const student = row.original
      return (
        <div className="flex items-center gap-3 pl-4">
          <Avatar className="size-8">
            <AvatarImage
              src={student.profile_photo_url || `https://api.dicebear.com/7.x/avataaars/svg?seed=${student.email || student.id}`}
              alt={student.name_english}
            />
            <AvatarFallback>
              {student.name_english
                .split(' ')
                .map((n) => n[0])
                .join('')
                .toUpperCase()}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <span className="font-medium">{student.name_english}</span>
            <span className="text-xs text-muted-foreground">
              {student.admission_number}
            </span>
          </div>
        </div>
      )
    },
  },
  {
    accessorKey: 'email',
    header: ({ column }) => (
      <Button
        variant="ghost"
        onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
      >
        Email
        {column.getIsSorted() === 'asc' ? (
          <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
        ) : column.getIsSorted() === 'desc' ? (
          <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
        ) : null}
      </Button>
    ),
    cell: ({ row }) => (
      <span className="pl-4">{row.getValue('email') || '-'}</span>
    ),
  },
  {
    accessorKey: 'status',
    header: ({ column }) => (
      <Button
        variant="ghost"
        onClick={() => column.toggleSorting(column.getIsSorted() === 'asc')}
      >
        Status
        {column.getIsSorted() === 'asc' ? (
          <HugeiconsIcon icon={ArrowUp01Icon} className="ml-2 h-4 w-4" />
        ) : column.getIsSorted() === 'desc' ? (
          <HugeiconsIcon icon={ArrowDown01Icon} className="ml-2 h-4 w-4" />
        ) : null}
      </Button>
    ),
    cell: ({ row }) => {
      const status = row.original.status
      return (
        <div className="pl-4">
          <Badge
            variant="secondary"
            className={
              status === 'Active'
                ? 'bg-green-500/10 text-green-600 border-green-500/20'
                : 'bg-orange-500/10 text-orange-600 border-orange-500/20'
            }
          >
            {status}
          </Badge>
        </div>
      )
    },
  },
  {
    accessorKey: 'profile_phone',
    header: 'Phone',
    cell: ({ row }) => <span className="pl-4">{row.getValue('profile_phone') || '-'}</span>,
  },
  {
    accessorKey: 'gender',
    header: 'Gender',
    cell: ({ row }) => <span className="pl-4">{row.getValue('gender')}</span>,
  },
  {
    id: 'actions',
    header: 'Actions',
    cell: ({ row }) => {
      const student = row.original
      return (
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button variant="ghost" size="icon" className="size-8">
                <HugeiconsIcon icon={Menu01Icon} className="size-4" />
              </Button>
            }
          />
          <DropdownMenuContent align="end" className="w-52">
            <DropdownMenuItem onClick={() => onEdit(student)}>
              <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
              Edit Profile
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onUploadPhoto(student)}>
              <HugeiconsIcon icon={Upload01Icon} className="mr-2 size-4" />
              Upload Photo
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => onAssignClass(student)}>
              <HugeiconsIcon icon={School01Icon} className="mr-2 size-4" />
              Assign Class
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onManageGuardians(student)}>
              <HugeiconsIcon icon={UserGroupIcon} className="mr-2 size-4" />
              Guardians
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => onManageAttendance(student)}>
              <HugeiconsIcon icon={CalendarCheckIn01Icon} className="mr-2 size-4" />
              Attendance
            </DropdownMenuItem>
            <DropdownMenuItem onClick={() => onManageMarks(student)}>
              <HugeiconsIcon icon={Chart01Icon} className="mr-2 size-4" />
              Academic Marks
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              className="text-destructive focus:text-destructive"
              onClick={() => onDelete(student.id)}
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
