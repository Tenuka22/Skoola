import { HugeiconsIcon } from '@hugeicons/react'
import {
  BookOpen01Icon,
  Delete02Icon,
  LayoutGridIcon,
  Link01Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import type { SubjectResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { DataTableColumnHeader } from '@/components/data-table'

interface GetSubjectsColumnsProps {
  onEdit: (subject: SubjectResponse) => void
  onDelete: (id: string) => void
  onAssignToGrade: (subject: SubjectResponse) => void
  onAssignToStream: (subject: SubjectResponse) => void
  onEnrollStudent: (subject: SubjectResponse) => void
  onViewEnrollments: (subject: SubjectResponse) => void
}

export function getSubjectsColumns({
  onEdit,
  onDelete,
  onAssignToGrade,
  onAssignToStream,
  onEnrollStudent,
  onViewEnrollments,
}: GetSubjectsColumnsProps): Array<DataTableColumnDef<SubjectResponse>> {
  return [
    {
      accessorKey: 'subject_name_en',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Name" />
      ),
      cell: ({ row }) => (
        <div className="font-medium">{row.original.subject_name_en}</div>
      ),
      meta: { isPinned: 'left' },
    },
    {
      accessorKey: 'subject_code',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Code" />
      ),
      cell: ({ row }) => (
        <Badge variant="outline">{row.original.subject_code}</Badge>
      ),
    },
    {
      accessorKey: 'is_core',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Type" />
      ),
      cell: ({ row }) => (
        <Badge variant={row.original.is_core ? 'default' : 'secondary'}>
          {row.original.is_core ? 'Core' : 'Elective'}
        </Badge>
      ),
    },
    {
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const subject = row.original

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
            <DropdownMenuContent align="end" className="w-[200px]">
              <DropdownMenuItem onClick={() => onEdit(subject)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="size-4 mr-2"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onDelete(subject.id)}>
                <HugeiconsIcon icon={Delete02Icon} className="size-4 mr-2" />
                Delete
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => onAssignToGrade(subject)}>
                <HugeiconsIcon icon={LayoutGridIcon} className="size-4 mr-2" />
                Assign to Grade
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onAssignToStream(subject)}>
                <HugeiconsIcon icon={BookOpen01Icon} className="size-4 mr-2" />
                Assign to Stream
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => onEnrollStudent(subject)}>
                <HugeiconsIcon icon={UserGroupIcon} className="size-4 mr-2" />
                Enroll Student
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onViewEnrollments(subject)}>
                <HugeiconsIcon icon={Link01Icon} className="size-4 mr-2" />
                View Enrollments
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
      meta: { isPinned: 'right' },
    },
  ]
}
