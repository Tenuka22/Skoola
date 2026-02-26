import { getStatusColor } from './staff-attendance-columns'
import type { CellContext, ColumnDef } from '@tanstack/react-table'
import type { StudentAttendanceWithMember } from '../types'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

export const studentAttendanceColumns: Array<
  ColumnDef<StudentAttendanceWithMember, unknown>
> = [
  {
    id: 'student_name',
    header: 'Student Name',
    cell: ({ row }: CellContext<StudentAttendanceWithMember, unknown>) => {
      const student = row.original.student
      return (
        <div className="flex items-center gap-3">
          <Avatar className="size-8 rounded-lg">
            <AvatarImage alt={student?.name_english} />
            <AvatarFallback className="rounded-lg">
              {student?.name_english?.charAt(0) ?? 'S'}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <span className="text-sm font-bold">{student?.name_english}</span>
            <span className="text-[10px] text-muted-foreground font-medium uppercase tracking-tight">
              {student?.admission_number}
            </span>
          </div>
        </div>
      )
    },
  },
  {
    accessorKey: 'status',
    header: 'Status',
    cell: ({ row }: CellContext<StudentAttendanceWithMember, unknown>) => {
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
    accessorKey: 'remarks',
    header: 'Note',
    cell: ({ row }: CellContext<StudentAttendanceWithMember, unknown>) => (
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
    cell: ({
      row,
      column,
    }: CellContext<StudentAttendanceWithMember, unknown>) => {
      const onMarkAttendance = column.columnDef.meta?.onMarkAttendance
      return onMarkAttendance ? (
        <Button
          variant="ghost"
          size="sm"
          className="h-8 font-bold text-[10px] uppercase tracking-wider hover:bg-primary hover:text-primary-foreground"
          onClick={() => onMarkAttendance(row.original)}
        >
          {row.original.id && !row.original.id.startsWith('temp-')
            ? 'Edit'
            : 'Mark'}
        </Button>
      ) : null
    },
  },
]
