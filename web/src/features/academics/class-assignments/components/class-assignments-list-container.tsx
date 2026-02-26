import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import { useClassAssignmentsStore } from '../store'
import type { ColumnDef } from '@tanstack/react-table'
import type { SubjectResponse } from '@/lib/api/types.gen'
import type { ClassAssignmentRow } from './class-assignments-table-columns'
import type { UseQueryResult } from '@tanstack/react-query'
import { DataTable } from '@/components/ui/data-table'

interface ClassAssignmentsListContainerProps {
  query: UseQueryResult<Array<SubjectResponse>, Error>
  columns: Array<ColumnDef<ClassAssignmentRow, unknown>>
  data: Array<ClassAssignmentRow>
}

export function ClassAssignmentsListContainer({
  query,
  columns,
  data,
}: ClassAssignmentsListContainerProps) {
  const { page, setPage, sorting, setSorting } = useClassAssignmentsStore()
  const { isLoading, isError, error } = query

  if (isLoading) {
    return (
      <div className="grid flex-1 place-items-center">
        <p className="text-sm text-muted-foreground">Loading assignments...</p>
      </div>
    )
  }

  if (isError) {
    return (
      <div className="grid flex-1 place-items-center px-4 py-8 text-center">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-destructive"
        />
        <p className="text-sm text-muted-foreground mt-2">
          Error: {error.message}
        </p>
      </div>
    )
  }

  if (data.length === 0) {
    return (
      <div className="grid flex-1 place-items-center px-4 py-8 text-center">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-muted-foreground"
        />
        <p className="text-sm text-muted-foreground mt-2">
          No assignments found for the selected class and academic year.
        </p>
      </div>
    )
  }

  const pageSize = 10
  const pageCount = Math.ceil(data.length / pageSize)

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto">
        <DataTable
          columns={columns}
          data={data}
          sorting={sorting}
          onSortingChange={setSorting}
          pageIndex={page - 1}
          pageSize={pageSize}
          pageCount={pageCount}
          canNextPage={page < pageCount}
          canPreviousPage={page > 1}
          fetchNextPage={() => setPage(page + 1)}
          fetchPreviousPage={() => setPage(page - 1)}
          isLoading={isLoading}
        />
      </div>
    </div>
  )
}
