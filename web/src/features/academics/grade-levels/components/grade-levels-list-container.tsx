import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import { useGradeLevelsStore } from '../store'
import type { ColumnDef } from '@tanstack/react-table'
import type {
  GradeLevelResponse,
  PaginatedGradeLevelResponse,
} from '@/lib/api/types.gen'
import type { UseQueryResult } from '@tanstack/react-query'
import { DataTable } from '@/components/ui/data-table'

interface GradeLevelsListContainerProps {
  query: UseQueryResult<PaginatedGradeLevelResponse, Error>
  columns: Array<ColumnDef<GradeLevelResponse>>
  rowSelection: Record<string, boolean>
  setRowSelection: React.Dispatch<React.SetStateAction<Record<string, boolean>>>
}

export function GradeLevelsListContainer({
  query,
  columns,
  rowSelection,
  setRowSelection,
}: GradeLevelsListContainerProps) {
  const { page, setPage, sorting, setSorting } = useGradeLevelsStore()
  const { data, isLoading, isError, error } = query

  if (isLoading) {
    return (
      <div className="grid flex-1 place-items-center">
        <p className="text-sm text-muted-foreground">Loading grade levels...</p>
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

  if (!data || data.data.length === 0) {
    return (
      <div className="grid flex-1 place-items-center px-4 py-8 text-center">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-muted-foreground"
        />
        <p className="text-sm text-muted-foreground mt-2">
          No grade levels found.
        </p>
      </div>
    )
  }

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto">
        <DataTable
          columns={columns}
          data={data.data}
          sorting={sorting}
          onSortingChange={setSorting}
          rowSelection={rowSelection}
          onRowSelectionChange={setRowSelection}
          pageIndex={page - 1}
          pageSize={data.limit}
          pageCount={data.total_pages}
          canNextPage={page < data.total_pages}
          canPreviousPage={page > 1}
          fetchNextPage={() => setPage(page + 1)}
          fetchPreviousPage={() => setPage(page - 1)}
          isLoading={isLoading}
        />
      </div>
    </div>
  )
}
