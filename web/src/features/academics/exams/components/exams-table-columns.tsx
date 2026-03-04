import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import { format } from 'date-fns'
import type { ExamResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'

import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { DataTableColumnHeader } from '@/components/data-table'

interface GetExamsColumnsProps {
  onEdit: (exam: ExamResponse) => void
  onDelete: (id: string) => void
}

export function getExamsColumns({
  onEdit,
  onDelete,
}: GetExamsColumnsProps): Array<DataTableColumnDef<ExamResponse>> {
  return [
    {
      accessorKey: 'name',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Exam Name" />
      ),
      cell: ({ row }) => <div className="font-medium">{row.original.name}</div>,
    },
    {
      accessorKey: 'start_date',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Start Date" />
      ),
      cell: ({ row }) => format(new Date(row.original.start_date), 'PP'),
    },
    {
      accessorKey: 'end_date',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="End Date" />
      ),
      cell: ({ row }) => format(new Date(row.original.end_date), 'PP'),
    },
    {
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const exam = row.original

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
              <DropdownMenuItem onClick={() => onEdit(exam)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="size-4 mr-2"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onDelete(exam.id)}>
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
