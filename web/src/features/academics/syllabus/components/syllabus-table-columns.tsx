import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  MoreVerticalIcon,
  PencilEdit01Icon,
  PlusSignIcon,
} from '@hugeicons/core-free-icons'
import type { SyllabusResponse } from '@/lib/api/types.gen'
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

interface GetSyllabusColumnsProps {
  onEdit: (topic: SyllabusResponse) => void
  onDelete: (id: string) => void
  onAddSubTopic: (parent: SyllabusResponse) => void
}

export function getSyllabusColumns({
  onEdit,
  onDelete,
  onAddSubTopic,
}: GetSyllabusColumnsProps): Array<DataTableColumnDef<SyllabusResponse>> {
  return [
    {
      accessorKey: 'topic_name',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Topic Name" />
      ),
      cell: ({ row }) => (
        <div className="font-medium">{row.original.topic_name}</div>
      ),
    },
    {
      accessorKey: 'required_periods',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Required Periods" />
      ),
      cell: ({ row }) => (
        <Badge variant="outline">{row.original.required_periods}</Badge>
      ),
    },
    {
      accessorKey: 'buffer_periods',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Buffer" />
      ),
      cell: ({ row }) => (
        <Badge variant="secondary">{row.original.buffer_periods}</Badge>
      ),
    },
    {
      accessorKey: 'is_practical',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Practical" />
      ),
      cell: ({ row }) => (
        <Badge variant={row.original.is_practical ? 'default' : 'outline'}>
          {row.original.is_practical ? 'Yes' : 'No'}
        </Badge>
      ),
    },
    {
      id: 'actions',
      header: 'Actions',
      cell: ({ row }) => {
        const topic = row.original

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
              <DropdownMenuItem onClick={() => onAddSubTopic(topic)}>
                <HugeiconsIcon icon={PlusSignIcon} className="size-4 mr-2" />
                Add Sub-topic
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onEdit(topic)}>
                <HugeiconsIcon
                  icon={PencilEdit01Icon}
                  className="size-4 mr-2"
                />
                Edit
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => onDelete(topic.id)}>
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
