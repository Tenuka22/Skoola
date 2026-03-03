import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import type { ClassResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllGradeLevelsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { DataTableColumnHeader } from '@/components/data-table'

interface GetClassesColumnsProps {
  onEdit: (classItem: ClassResponse) => void
  onDelete: (id: string) => void
  onAssignStudents: (classItem: ClassResponse) => void
}

export function useClassesColumns({
  onEdit,
  onDelete,
  onAssignStudents,
}: GetClassesColumnsProps): Array<DataTableColumnDef<ClassResponse>> {
  const { data: academicYearsData } = useQuery(
    getAllAcademicYearsOptions({ client: authClient }),
  )
  const academicYears = academicYearsData?.data || []

  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = gradeLevelsData?.data || []

  return [
    {
      accessorKey: 'section_name',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Name" />
      ),
      cell: ({ row }) => (
        <div className="font-medium">{row.original.section_name}</div>
      ),
    },
    {
      accessorKey: 'grade_id',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Grade Level" />
      ),
      cell: ({ row }) => {
        const gradeLevel = gradeLevels.find(
          (gl) => gl.id === row.original.grade_id,
        )
        return (
          <Badge variant="outline">
            {gradeLevel?.grade_name || row.original.grade_id}
          </Badge>
        )
      },
    },
    {
      accessorKey: 'academic_year_id',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Academic Year" />
      ),
      cell: ({ row }) => {
        const academicYear = academicYears.find(
          (ay) => ay.id === row.original.academic_year_id,
        )
        return (
          <Badge variant="secondary">
            {academicYear?.name || row.original.academic_year_id}
          </Badge>
        )
      },
    },
    {
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const classItem = row.original

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
              <DropdownMenuItem onClick={() => onEdit(classItem)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="size-4 mr-2"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onAssignStudents(classItem)}>
                <HugeiconsIcon icon={UserGroupIcon} className="size-4 mr-2" />
                Assign Students
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onDelete(classItem.id)}>
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
