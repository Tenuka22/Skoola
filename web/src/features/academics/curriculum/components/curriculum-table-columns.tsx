import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  Link01Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import { useQuery } from '@tanstack/react-query'
import { Link } from '@tanstack/react-router'
import type { CurriculumStandardResponse } from '@/lib/api/types.gen'
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
  getAllGradeLevelsOptions,
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { DataTableColumnHeader } from '@/components/data-table'

interface GetCurriculumColumnsProps {
  onEdit: (standard: CurriculumStandardResponse) => void
  onDelete: (id: string) => void
}

export function useCurriculumColumns({
  onEdit,
  onDelete,
}: GetCurriculumColumnsProps): Array<
  DataTableColumnDef<CurriculumStandardResponse>
> {
  const { data: subjectsData } = useQuery(
    getAllSubjectsOptions({ client: authClient }),
  )
  const subjects = subjectsData?.data || []

  const { data: gradeLevelsData } = useQuery(
    getAllGradeLevelsOptions({ client: authClient }),
  )
  const gradeLevels = gradeLevelsData?.data || []

  return [
    {
      accessorKey: 'standard_code',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Code" />
      ),
      cell: ({ row }) => (
        <div className="font-mono text-xs font-semibold">
          {row.original.standard_code}
        </div>
      ),
    },
    {
      accessorKey: 'subject_id',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Subject" />
      ),
      cell: ({ row }) => {
        const subject = subjects.find((s) => s.id === row.original.subject_id)
        return <div>{subject?.subject_name_en || row.original.subject_id}</div>
      },
    },
    {
      accessorKey: 'grade_level_id',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Grade Level" />
      ),
      cell: ({ row }) => {
        const gradeLevel = gradeLevels.find(
          (gl) => gl.id === row.original.grade_level_id,
        )
        return (
          <Badge variant="outline">
            {gradeLevel?.grade_name || row.original.grade_level_id}
          </Badge>
        )
      },
    },
    {
      accessorKey: 'medium',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Medium" />
      ),
      cell: ({ row }) => (
        <Badge variant="secondary">{row.original.medium}</Badge>
      ),
    },
    {
      accessorKey: 'version_name',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Version" />
      ),
      cell: ({ row }) => row.original.version_name || '-',
    },
    {
      accessorKey: 'is_active',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Status" />
      ),
      cell: ({ row }) => (
        <Badge variant={row.original.is_active ? 'default' : 'destructive'}>
          {row.original.is_active ? 'Active' : 'Inactive'}
        </Badge>
      ),
    },
    {
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const standard = row.original

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
            <DropdownMenuContent align="end" className="w-[180px]">
              <DropdownMenuItem>
                <Link
                  to="/admin/academics/syllabus"
                  search={{ standardId: standard.id }}
                  className="flex items-center px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50"
                >
                  <HugeiconsIcon icon={Link01Icon} className="size-4 mr-2" />
                  View Syllabus
                </Link>
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onEdit(standard)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="size-4 mr-2"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onDelete(standard.id)}>
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
