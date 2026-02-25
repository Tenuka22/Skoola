import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import { useTimetablesStore } from '../store'
import type { ColumnDef } from '@tanstack/react-table'
import type { TimetableResponse } from '@/lib/api/types.gen'
import type { TimetableEntryRow } from './timetables-table-columns'
import type { UseQueryResult } from '@tanstack/react-query'
import { DataTable } from '@/components/ui/data-table'

interface TimetablesListContainerProps {
  query: UseQueryResult<Array<TimetableResponse>, Error>
  columns: Array<ColumnDef<TimetableEntryRow>>
  data: Array<TimetableEntryRow>
}

export function TimetablesListContainer({
  query,
  columns,
  data,
}: TimetablesListContainerProps) {
  const { page, sorting, setSorting } = useTimetablesStore()
  const { isLoading, isError, error } = query

  if (isLoading) {
    return (
      <div className="grid flex-1 place-items-center py-8">
        <p className="text-sm text-muted-foreground">Loading timetables...</p>
      </div>
    )
  }

  if (isError) {
    return (
      <div className="grid flex-1 place-items-center px-4 py-8 text-center border rounded-lg">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-destructive"
        />
        <p className="text-sm text-muted-foreground mt-2">
          Error: {error?.message}
        </p>
      </div>
    )
  }

  if (data.length === 0) {
    return (
      <div className="grid flex-1 place-items-center px-4 py-8 text-center border rounded-lg">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-muted-foreground opacity-20"
        />
        <p className="text-sm text-muted-foreground mt-2">
          No timetable entries found for the selected criteria.
        </p>
      </div>
    )
  }

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto">
        <DataTable
          columns={columns}
          data={data}
          sorting={sorting}
          onSortingChange={setSorting as any}
          pageIndex={page - 1}
          pageSize={data.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
          isLoading={isLoading}
        />
      </div>
    </div>
  )
}
