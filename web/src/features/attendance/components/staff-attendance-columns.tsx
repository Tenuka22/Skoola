import { HugeiconsIcon } from '@hugeicons/react'
import { Image01Icon, Location01Icon } from '@hugeicons/core-free-icons'
import type { ColumnDef } from '@tanstack/react-table'
import type { StaffAttendanceWithMember } from '../types'
import type { AttendanceStatus } from '@/lib/api/types.gen'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

export const getStatusColor = (status: AttendanceStatus) => {
  switch (status) {
    case 'Present':
      return 'text-green-500 bg-green-500/10 border-green-500/20'
    case 'Absent':
      return 'text-red-500 bg-red-500/10 border-red-500/20'
    case 'Late':
      return 'text-orange-500 bg-orange-500/10 border-orange-500/20'
    case 'Excused':
      return 'text-blue-500 bg-blue-500/10 border-blue-500/20'
    case 'HalfDay':
      return 'text-purple-500 bg-purple-500/10 border-purple-500/20'
    default:
      return 'text-muted-foreground bg-muted/10 border-muted/20'
  }
}

export const staffAttendanceColumns: Array<
  ColumnDef<StaffAttendanceWithMember>
> = [
  {
    id: 'staff_name',
    header: 'Employee Name',
    cell: ({ row }) => {
      const staff = row.original.staff
      return (
        <div className="flex items-center gap-3">
          <Avatar className="size-8 rounded-lg">
            <AvatarImage src={staff?.photo_url ?? ''} alt={staff?.name} />
            <AvatarFallback className="rounded-lg">
              {staff?.name?.charAt(0) ?? 'S'}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <span className="text-sm font-bold">{staff?.name}</span>
            <span className="text-[10px] text-muted-foreground font-medium uppercase tracking-tight">
              {staff?.employee_id}
            </span>
          </div>
        </div>
      )
    },
  },
  {
    id: 'clock_in_out',
    header: 'Clock-in & Out',
    cell: ({ row }) => {
      const { time_in, time_out } = row.original
      if (!time_in) return <span className="text-muted-foreground">-</span>
      return (
        <div className="flex items-center gap-2">
          <span
            className={cn(
              'text-sm font-bold',
              row.original.status === 'Late'
                ? 'text-orange-500'
                : 'text-primary',
            )}
          >
            {time_in}
          </span>
          <div className="flex items-center gap-1">
            <div className="h-[2px] w-4 bg-muted" />
            <span className="text-[10px] text-muted-foreground font-medium">
              8h 58m
            </span>
            <div className="h-[2px] w-4 bg-muted" />
          </div>
          <span className="text-sm font-bold text-orange-500">
            {time_out ?? '-'}
          </span>
        </div>
      )
    },
  },
  {
    accessorKey: 'status',
    header: 'Status',
    cell: ({ row }) => {
      const status = row.original.status
      return (
        <Badge
          variant="outline"
          className={cn('rounded-lg font-bold border', getStatusColor(status))}
        >
          {status}
        </Badge>
      )
    },
  },
  {
    id: 'overtime',
    header: 'Overtime',
    cell: () => <span className="text-sm font-bold">-</span>,
  },
  {
    id: 'picture',
    header: 'Picture',
    cell: () => (
      <div className="flex items-center gap-1 text-primary cursor-pointer hover:underline">
        <HugeiconsIcon icon={Image01Icon} className="size-3" />
        <span className="text-[10px] font-bold">view_photo...</span>
      </div>
    ),
  },
  {
    id: 'location',
    header: 'Location',
    cell: () => (
      <div className="flex items-center gap-1 text-primary cursor-pointer hover:underline text-nowrap">
        <HugeiconsIcon icon={Location01Icon} className="size-3" />
        <span className="text-[10px] font-bold">Jl. Jendral Sudirma...</span>
      </div>
    ),
  },
  {
    accessorKey: 'remarks',
    header: 'Note',
    cell: ({ row }) => (
      <div className="flex items-center gap-1 max-w-[200px] truncate">
        <span className="text-[10px] font-medium text-muted-foreground">
          {row.original.remarks ?? 'No remarks'}
        </span>
      </div>
    ),
  },
  {
    id: 'actions',
    header: '',
    cell: ({ row, column }) => {
      return (column.columnDef.meta as any)?.onMarkAttendance ? (
        <Button
          variant="ghost"
          size="sm"
          className="h-8 font-bold text-[10px] uppercase tracking-wider hover:bg-primary hover:text-primary-foreground"
          onClick={() =>
            (column.columnDef.meta as any).onMarkAttendance(row.original)
          }
        >
          {row.original.created_at ? 'Edit' : 'Mark'}
        </Button>
      ) : null
    },
  },
]
