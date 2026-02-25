'use client'

import {
  Calendar02Icon,
  Delete02Icon,
  MoreHorizontalIcon,
  PencilEdit01Icon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { format } from 'date-fns'
import type { ColumnDef } from '@tanstack/react-table'
import type { AcademicYearResponse } from '@/lib/api/types.gen'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Checkbox } from '@/components/ui/checkbox'
import { Badge } from '@/components/ui/badge'

interface GetAcademicYearsColumnsProps {
  onEdit: (year: AcademicYearResponse) => void
  onDelete: (id: string) => void
  onSetCurrent: (id: string) => void
}

export const getAcademicYearsColumns = ({
  onEdit,
  onDelete,
  onSetCurrent,
}: GetAcademicYearsColumnsProps): Array<ColumnDef<AcademicYearResponse>> => [
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
    accessorKey: 'name',
    header: 'Academic Year',
    cell: ({ row }) => {
      const year = row.original
      return (
        <div className="flex flex-col">
          <span className="font-medium">{year.name}</span>
          <span className="text-xs text-muted-foreground">
            {format(new Date(String(year.year_start)), 'd MMM yyyy')} -{' '}
            {format(new Date(String(year.year_end)), 'd MMM yyyy')}
          </span>
        </div>
      )
    },
  },
  {
    accessorKey: 'current',
    header: 'Status',
    cell: ({ row }) => {
      const isCurrent = row.original.current
      return isCurrent ? <Badge>Current</Badge> : null
    },
  },
  {
    id: 'actions',
    cell: ({ row }) => {
      const year = row.original
      return (
        <DropdownMenu>
          <DropdownMenuTrigger
            render={
              <Button variant="ghost" size="icon" className="size-8">
                <HugeiconsIcon icon={MoreHorizontalIcon} className="size-4" />
              </Button>
            }
          />
          <DropdownMenuContent align="end" className="w-48">
            <DropdownMenuItem onClick={() => onEdit(year)}>
              <HugeiconsIcon icon={PencilEdit01Icon} className="mr-2 size-4" />
              Edit
            </DropdownMenuItem>
            {!year.current && (
              <DropdownMenuItem onClick={() => onSetCurrent(year.id)}>
                <HugeiconsIcon icon={Calendar02Icon} className="mr-2 size-4" />
                Set as Current
              </DropdownMenuItem>
            )}
            <DropdownMenuItem
              className="text-destructive focus:text-destructive"
              onClick={() => onDelete(year.id)}
            >
              <HugeiconsIcon icon={Delete02Icon} className="mr-2 size-4" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )
    },
  },
]
