import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import type { ColumnDef } from '@tanstack/react-table'
import type { GradeLevelResponse } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { DataTableColumnHeader } from '@/components/ui/data-table-column-header'
import { Checkbox } from '@/components/ui/checkbox'

interface GetGradeLevelsColumnsProps {
  onEdit: (gradeLevel: GradeLevelResponse) => void
  onDelete: (id: string) => void
}

export function getGradeLevelsColumns({
  onEdit,
  onDelete,
}: GetGradeLevelsColumnsProps): Array<ColumnDef<GradeLevelResponse>> {
  return [
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
      accessorKey: 'grade_name',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Name" />
      ),
      cell: ({ row }) => (
        <div className="font-medium">{row.original.grade_name}</div>
      ),
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
    },
  ]
}
