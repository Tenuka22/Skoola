import { HugeiconsIcon } from '@hugeicons/react'
import { AlertCircleIcon } from '@hugeicons/core-free-icons'
import { useClassesSearchParams } from '../search-params'
import type { ColumnDef } from '@tanstack/react-table'
import type { ClassResponse, PaginatedClassResponse } from '@/lib/api/types.gen'
import type { UseQueryResult } from '@tanstack/react-query'
import { DataTable } from '@/components/ui/data-table'
import { Stack, Text } from '@/components/primitives'

interface ClassesListContainerProps {
  query: UseQueryResult<PaginatedClassResponse, Error>
  columns: Array<ColumnDef<ClassResponse, unknown>>
  rowSelection: Record<string, boolean>
  setRowSelection: React.Dispatch<React.SetStateAction<Record<string, boolean>>>
}

export function ClassesListContainer({
  query,
  columns,
  rowSelection,
  setRowSelection,
}: ClassesListContainerProps) {
  const { page, setPage, sortBy, setSortBy, sortOrder, setSortOrder } =
    useClassesSearchParams()
  const { data, isLoading, isError, error } = query

  if (isLoading) {
    return (
      <Stack align="center" justify="center" className="flex-1">
        <Text size="sm" muted>
          Loading classes...
        </Text>
      </Stack>
    )
  }

  if (isError) {
    return (
      <Stack align="center" justify="center" gap={2} className="flex-1">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-destructive"
        />
        <Text size="sm" muted>
          Error: {error.message}
        </Text>
      </Stack>
    )
  }

  if (!data || data.data.length === 0) {
    return (
      <Stack align="center" justify="center" gap={2} className="flex-1">
        <HugeiconsIcon
          icon={AlertCircleIcon}
          className="size-12 text-muted-foreground"
        />
        <Text size="sm" muted>
          No classes found.
        </Text>
      </Stack>
    )
  }

  return (
    <div className="overflow-y-auto flex-1">
      <DataTable
        columns={columns}
        data={data.data}
        sorting={[{ id: sortBy ?? 'created_at', desc: sortOrder === 'desc' }]}
        onSortingChange={(updaterOrValue) => {
          const newSorting =
            typeof updaterOrValue === 'function'
              ? updaterOrValue([
                  { id: sortBy ?? 'created_at', desc: sortOrder === 'desc' },
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
  )
}
