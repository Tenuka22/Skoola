import { ColumnDef } from '@tanstack/react-table'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  MoreHorizontalIcon,
  PencilEdit01Icon,
  Delete02Icon,
  Clock01Icon,
} from '@hugeicons/core-free-icons'
import type { GradePeriodResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Badge } from '@/components/ui/badge'
import { HStack, Text } from '@/components/primitives'
import { DataTableColumnHeader } from '@/components/data-table'

interface GradePeriodsColumnsProps {
  onEdit: (period: GradePeriodResponse) => void
  onDelete: (period: GradePeriodResponse) => void
}

export const getGradePeriodsColumns = ({
  onEdit,
  onDelete,
}: GradePeriodsColumnsProps): ColumnDef<GradePeriodResponse>[] => [
  {
    accessorKey: 'period_number',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Period" />
    ),
    cell: ({ row }) => (
      <div className="flex items-center gap-2">
        <Badge variant="outline" className="h-6 w-6 flex items-center justify-center p-0 rounded-full">
          {row.getValue('period_number')}
        </Badge>
      </div>
    ),
  },
  {
    accessorKey: 'start_time',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Start Time" />
    ),
    cell: ({ row }) => (
      <HStack gap={2}>
        <HugeiconsIcon icon={Clock01Icon} className="size-3.5 text-muted-foreground" />
        <Text size="sm">{row.getValue('start_time')}</Text>
      </HStack>
    ),
  },
  {
    accessorKey: 'end_time',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="End Time" />
    ),
    cell: ({ row }) => (
      <HStack gap={2}>
        <HugeiconsIcon icon={Clock01Icon} className="size-3.5 text-muted-foreground" />
        <Text size="sm">{row.getValue('end_time')}</Text>
      </HStack>
    ),
  },
  {
    accessorKey: 'is_break',
    header: ({ column }) => (
      <DataTableColumnHeader column={column} title="Type" />
    ),
    cell: ({ row }) => {
      const isBreak = row.getValue('is_break') as boolean
      return (
        <Badge variant={isBreak ? 'secondary' : 'default'} className="text-[10px] px-1.5 py-0">
          {isBreak ? 'Break' : 'Lesson'}
        </Badge>
      )
    },
  },
  {
    id: 'actions',
    cell: ({ row }) => {
      const period = row.original

      return (
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button variant="ghost" className="h-8 w-8 p-0">
                <span className="sr-only">Open menu</span>
                <HugeiconsIcon icon={MoreHorizontalIcon} className="h-4 w-4" />
              </Button>
            }
          />
          <DropdownMenuContent align="end">
            <DropdownMenuLabel>Actions</DropdownMenuLabel>
            <DropdownMenuItem onClick={() => onEdit(period)}>
              <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 h-4 w-4" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              onClick={() => onDelete(period)}
              className="text-destructive focus:text-destructive"
            >
              <HugeiconsIcon icon={Delete02Icon} className="mr-2 h-4 w-4" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )
    },
  },
]
