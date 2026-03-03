import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useSubjectsSearchParams } from '../search-params'
import type { ColumnDef } from '@tanstack/react-table'
import type {
  PaginatedSubjectResponse,
  SubjectResponse,
} from '@/lib/api/types.gen'
import type { UseQueryResult } from '@tanstack/react-query'
import { DataTable } from '@/components/ui/data-table'

interface SubjectsListContainerProps {
  query: UseQueryResult<PaginatedSubjectResponse, Error>
  columns: Array<ColumnDef<SubjectResponse>>
  rowSelection: Record<string, boolean>
  setRowSelection: React.Dispatch<React.SetStateAction<Record<string, boolean>>>
}

export function SubjectsListContainer({
  query,
  columns,
  rowSelection,
  setRowSelection,
}: SubjectsListContainerProps) {
  const { page, setPage, sortBy, setSortBy, sortOrder, setSortOrder } =
    useSubjectsSearchParams()
  const { data, isLoading, isError, error } = query

  if (isLoading) {
    return (
      <div className="grid flex-1 place-items-center py-8">
        <p className="text-sm text-muted-foreground">Loading subjects...</p>
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
          Error: {error.message}
        </p>
      </div>
    )
  }

  if (!data || data.data.length === 0) {
    return (
      <div className="grid flex-1 place-items-center px-4 py-8 text-center border rounded-lg">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-muted-foreground"
        />
        <p className="text-sm text-muted-foreground mt-2">No subjects found.</p>
      </div>
    )
  }

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto">
        <DataTable
          columns={columns}
          data={data.data}
          sorting={[
            { id: sortBy ?? 'subject_name_en', desc: sortOrder === 'desc' },
          ]}
          onSortingChange={(updaterOrValue) => {
            const newSorting =
              typeof updaterOrValue === 'function'
                ? updaterOrValue([
                    {
                      id: sortBy ?? 'subject_name_en',
                      desc: sortOrder === 'desc',
                    },
                  ])
                : updaterOrValue
            const firstSort = newSorting[0]
            if (firstSort) {
              setSortBy(firstSort.id)
              setSortOrder(firstSort.desc ? 'desc' : 'asc')
            }
          }}
          rowSelection={rowSelection}
          onRowSelectionChange={setRowSelection}
          pageIndex={(page ?? 1) - 1}
          pageSize={data.limit}
          pageCount={data.total_pages}
          canNextPage={(page ?? 1) < data.total_pages}
          canPreviousPage={(page ?? 1) > 1}
          fetchNextPage={() => setPage((page ?? 1) + 1)}
          fetchPreviousPage={() => setPage((page ?? 1) - 1)}
          isLoading={isLoading}
        />
      </div>
    </div>
  )
}
