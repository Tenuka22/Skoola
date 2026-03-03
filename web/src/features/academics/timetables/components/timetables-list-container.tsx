import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { TimetablesVisualView } from './timetables-visual-view'
import type { ColumnDef, SortingState } from '@tanstack/react-table'
import type { TimetableResponse } from '@/lib/api/types.gen'
import type { TimetableEntryRow } from './timetables-table-columns'
import type { UseQueryResult } from '@tanstack/react-query'
import { DataTable } from '@/components/ui/data-table'

interface TimetablesListContainerProps {
  query: UseQueryResult<Array<TimetableResponse>, Error>
  columns: Array<ColumnDef<TimetableEntryRow>>
  data: Array<TimetableEntryRow>
  isGridView: boolean
  viewMode: string
  onEdit: (entry: TimetableResponse | null) => void
}

export function TimetablesListContainer({
  query,
  columns,
  data,
  isGridView,
  viewMode,
  onEdit,
}: TimetablesListContainerProps) {
  const [sorting, setSorting] = React.useState<SortingState>([])
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
      <div className="grid flex-1 place-items-center px-4 py-8 text-center border rounded-lg m-8">
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
      <div className="grid flex-1 place-items-center px-4 py-8 text-center border rounded-lg m-8">
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

  if (isGridView) {
    return (
      <TimetablesVisualView
        data={data}
        viewMode={viewMode}
        setTimetableEntryToEdit={onEdit}
      />
    )
  }

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto px-8 pb-8">
        <DataTable
          columns={columns}
          data={data}
          sorting={sorting}
          onSortingChange={setSorting}
          pageIndex={0}
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
