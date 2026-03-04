import { useQuery } from '@tanstack/react-query'
import { format } from 'date-fns'
import type { LessonProgressResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { Badge } from '@/components/ui/badge'
import { authClient } from '@/lib/clients'
import { getAllStaffOptions } from '@/lib/api/@tanstack/react-query.gen'
import { DataTableColumnHeader } from '@/components/data-table'

export function useLessonProgressColumns(): Array<
  DataTableColumnDef<LessonProgressResponse>
> {
  const { data: staffData } = useQuery(
    getAllStaffOptions({ client: authClient }),
  )
  const staff = staffData?.data || []

  return [
    {
      accessorKey: 'date',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Date" />
      ),
      cell: ({ row }) => (
        <div className="font-medium">
          {format(new Date(row.original.date), 'PP')}
        </div>
      ),
    },
    {
      accessorKey: 'teacher_id',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Teacher" />
      ),
      cell: ({ row }) => {
        const teacher = staff.find((s) => s.id === row.original.teacher_id)
        return <div>{teacher?.name || row.original.teacher_id}</div>
      },
    },
    {
      accessorKey: 'topic_covered',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Topic" />
      ),
      cell: ({ row }) => (
        <div
          className="max-w-[300px] truncate"
          title={row.original.topic_covered}
        >
          {row.original.topic_covered}
        </div>
      ),
    },
    {
      accessorKey: 'progress_percentage',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Progress" />
      ),
      cell: ({ row }) => (
        <Badge variant="outline">{row.original.progress_percentage}%</Badge>
      ),
    },
    {
      accessorKey: 'is_skipped',
      header: ({ column }) => (
        <DataTableColumnHeader column={column} title="Skipped" />
      ),
      cell: ({ row }) => (
        <Badge variant={row.original.is_skipped ? 'destructive' : 'secondary'}>
          {row.original.is_skipped ? 'Yes' : 'No'}
        </Badge>
      ),
    },
  ]
}
