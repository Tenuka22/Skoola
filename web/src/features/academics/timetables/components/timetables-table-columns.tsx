import { HugeiconsIcon } from '@hugeicons/react'
import { Delete02Icon, MoreVerticalIcon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import type { ColumnDef } from '@tanstack/react-table'
import type {
  AcademicYearResponse,
  ClassResponse,
  StaffResponse,
  SubjectResponse,
  TimetableResponse,
} from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { DataTableColumnHeader } from '@/components/ui/data-table-column-header'

export interface TimetableEntryRow extends Record<string, unknown> {
  id: string
  className: string
  subjectName: string
  teacherName: string
  dayOfWeek: string
  startTime: string
  endTime: string
  room: string
  academicYearName: string
  // Original IDs for mutations
  originalTimetableEntry: TimetableResponse
}

export function mapTimetableEntriesForTable(
  entries: Array<TimetableResponse>,
  academicYears: Array<AcademicYearResponse>,
  classes: Array<ClassResponse>,
  staff: Array<StaffResponse>,
  subjects: Array<SubjectResponse> = [],
): Array<TimetableEntryRow> {
  return entries.map((entry) => {
    const academicYear = academicYears.find(
      (ay) => ay.id === entry.academic_year_id,
    )
    const cls = classes.find((c) => c.id === entry.class_id)
    const teacher = staff.find((s) => s.id === entry.teacher_id)
    const subject = subjects.find((s) => s.id === entry.subject_id)

    return {
      id: entry.id,
      className: cls?.section_name || 'N/A',
      subjectName: subject?.subject_name_en || entry.subject_id,
      teacherName: teacher?.name || entry.teacher_id,
      dayOfWeek: entry.day_of_week,
      startTime: entry.start_time,
      endTime: entry.end_time,
      room: entry.room || 'N/A',
      academicYearName: academicYear?.name || 'N/A',
      originalTimetableEntry: entry,
    }
  })
}

interface GetTimetableColumnsProps {
  onEdit: (entry: TimetableResponse) => void
  onDelete: (id: string) => void
}

export function getTimetableColumns({
  onEdit,
  onDelete,
}: GetTimetableColumnsProps): Array<ColumnDef<TimetableEntryRow>> {
  return [
    {
      accessorKey: 'className',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Class" />
      ),
      cell: ({ row }) => (
        <Badge variant="outline">{row.original.className}</Badge>
      ),
    },
    {
      accessorKey: 'subjectName',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Subject" />
      ),
      cell: ({ row }) => (
        <Badge variant="secondary">{row.original.subjectName}</Badge>
      ),
    },
    {
      accessorKey: 'teacherName',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Teacher" />
      ),
      cell: ({ row }) => (
        <div className="font-medium">{row.original.teacherName}</div>
      ),
    },
    {
      accessorKey: 'dayOfWeek',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Day" />
      ),
      cell: ({ row }) => (
        <div className="text-muted-foreground">{row.original.dayOfWeek}</div>
      ),
    },
    {
      accessorKey: 'startTime',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Start Time" />
      ),
      cell: ({ row }) => (
        <div className="font-mono text-sm">{row.original.startTime}</div>
      ),
    },
    {
      accessorKey: 'endTime',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="End Time" />
      ),
      cell: ({ row }) => (
        <div className="font-mono text-sm">{row.original.endTime}</div>
      ),
    },
    {
      accessorKey: 'room',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Room" />
      ),
      cell: ({ row }) => (
        <div className="text-muted-foreground">{row.original.room}</div>
      ),
    },
    {
      accessorKey: 'academicYearName',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Academic Year" />
      ),
      cell: ({ row }) => (
        <div className="text-muted-foreground">
          {row.original.academicYearName}
        </div>
      ),
    },
    {
      id: 'actions',
      cell: ({ row }) => {
        const entry = row.original.originalTimetableEntry

        return (
          <DropdownMenu>
            <DropdownMenuTrigger
              render={
                <Button
                  variant="ghost"
                  className="flex size-8 p-0 data-[state=open]:bg-muted"
                >
                  <HugeiconsIcon icon={MoreVerticalIcon} className="size-4" />
                  <span className="sr-only">Open menu</span>
                </Button>
              }
            />
            <DropdownMenuContent align="end" className="w-[160px]">
              <DropdownMenuItem onClick={() => onEdit(entry)}>
                <HugeiconsIcon icon={PencilEdit01Icon} className="size-4 mr-2" />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onDelete(entry.id)}>
                <HugeiconsIcon icon={Delete02Icon} className="size-4 mr-2" />
                Delete
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
    },
  ]
}
