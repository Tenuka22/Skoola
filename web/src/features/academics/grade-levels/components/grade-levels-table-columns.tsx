import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import type { GradeLevelResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { DataTableColumnHeader } from '@/components/data-table'

interface GetGradeLevelsColumnsProps {
  onEdit: (gradeLevel: GradeLevelResponse) => void
  onDelete: (id: string) => void
}

export function getGradeLevelsColumns({
  onEdit,
  onDelete,
}: GetGradeLevelsColumnsProps): Array<DataTableColumnDef<GradeLevelResponse>> {
  return [
    {
      accessorKey: 'grade_name',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Name" />
      ),
      cell: ({ row }) => (
        <div className="font-medium">{row.original.grade_name}</div>
      ),
      meta: { isPinned: 'left' },
    },
    {
      accessorKey: 'grade_number',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Number" />
      ),
      cell: ({ row }) => (
        <Badge variant="secondary">{row.original.grade_number}</Badge>
      ),
    },
    {
      accessorKey: 'education_level',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Education Level" />
      ),
      cell: ({ row }) => (
        <div className="text-muted-foreground line-clamp-1 max-w-sm">
          {row.original.education_level || 'N/A'}
        </div>
      ),
    },
    {
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const gradeLevel = row.original

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
              <DropdownMenuItem onClick={() => onEdit(gradeLevel)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="size-4 mr-2"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onDelete(gradeLevel.id)}>
                <HugeiconsIcon icon={Delete02Icon} className="size-4 mr-2" />
                Delete
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
      meta: { isPinned: 'right' },
    },
  ]
}
