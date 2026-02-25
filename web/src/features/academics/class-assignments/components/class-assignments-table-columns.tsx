import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import type { ColumnDef } from '@tanstack/react-table'
import type {
  AcademicYearResponse,
  ClassResponse,
  ClassSubjectTeacherResponse,
  StaffResponse,
  SubjectResponse,
} from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

export interface ClassAssignmentRow {
  id: string
  className: string
  subjectName: string
  teacherName: string
  academicYearName: string
  // Original IDs for mutations
  class_id: string
  subject_id: string
  teacher_id: string
  academic_year_id: string
}

export function mapAssignmentsForTable(
  assignments: Array<ClassSubjectTeacherResponse>,
  academicYears: Array<AcademicYearResponse>,
  classes: Array<ClassResponse>,
  subjects: Array<SubjectResponse> = [],
  staff: Array<StaffResponse> = [],
): Array<ClassAssignmentRow> {
  return assignments.map((assignment) => {
    const academicYear = academicYears.find(
      (ay) => ay.id === assignment.academic_year_id,
    )
    const cls = classes.find((c) => c.id === assignment.class_id)
    const subject = subjects.find((s) => s.id === assignment.subject_id)
    const teacher = staff.find((s) => s.id === assignment.teacher_id)

    return {
      id: `${assignment.class_id}-${assignment.subject_id}-${assignment.teacher_id}-${assignment.academic_year_id}`,
      className: cls?.section_name || 'N/A',
      subjectName: subject?.subject_name_en || 'N/A',
      teacherName: teacher?.name || 'N/A',
      academicYearName: academicYear?.name || 'N/A',
      class_id: assignment.class_id,
      subject_id: assignment.subject_id,
      teacher_id: assignment.teacher_id,
      academic_year_id: assignment.academic_year_id,
    }
  })
}

interface GetAssignmentColumnsProps {
  onEdit: (assignment: ClassAssignmentRow) => void
  onDelete: (assignment: ClassAssignmentRow) => void
}

export function getAssignmentColumns({
  onEdit,
  onDelete,
}: GetAssignmentColumnsProps): Array<ColumnDef<ClassAssignmentRow>> {
  return [
    {
      accessorKey: 'className',
      header: 'Class',
      cell: ({ row }) => (
        <Badge variant="outline">{row.original.className}</Badge>
      ),
    },
    {
      accessorKey: 'subjectName',
      header: 'Subject',
      cell: ({ row }) => (
        <Badge variant="secondary">{row.original.subjectName}</Badge>
      ),
    },
    {
      accessorKey: 'teacherName',
      header: 'Teacher',
      cell: ({ row }) => (
        <div className="font-medium">{row.original.teacherName}</div>
      ),
    },
    {
      accessorKey: 'academicYearName',
      header: 'Academic Year',
      cell: ({ row }) => (
        <div className="text-muted-foreground">
          {row.original.academicYearName}
        </div>
      ),
    },
    {
      id: 'actions',
      cell: ({ row }) => {
        const assignment = row.original

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
              <DropdownMenuItem onClick={() => onEdit(assignment)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="size-4 mr-2"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onDelete(assignment)}>
                <HugeiconsIcon icon={Delete02Icon} className="size-4 mr-2" />
                Remove
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
    },
  ]
}
