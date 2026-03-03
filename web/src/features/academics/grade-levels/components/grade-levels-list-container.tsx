import * as React from 'react'
import { useGradeLevelsSearchParams } from '../search-params'
import type {
  GradeLevelResponse,
  PaginatedGradeLevelResponse,
} from '@/lib/api/types.gen'
import type { UseQueryResult } from '@tanstack/react-query'
import type {
  DataTableColumnDef,
  DataTableToolbarContext,
} from '@/components/data-table'
import type { Table } from '@tanstack/react-table'
import { DataTable } from '@/components/data-table'

interface GradeLevelsListContainerProps {
  query: UseQueryResult<PaginatedGradeLevelResponse, Error>
  columns: Array<DataTableColumnDef<GradeLevelResponse, unknown>>
  rowSelection: Record<string, boolean>
  setRowSelection: React.Dispatch<React.SetStateAction<Record<string, boolean>>>
  bulkActions?: (context: {
    selectedRows: Array<GradeLevelResponse>
    table: Table<GradeLevelResponse>
  }) => React.ReactNode
  onFetchFullData?: () => Promise<Array<GradeLevelResponse>>
  onAdd?: () => void
  onAddLabel?: string
  toolbar?: (
    context: DataTableToolbarContext<GradeLevelResponse>,
  ) => React.ReactNode
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  extraActions?: React.ReactNode
}

export function GradeLevelsListContainer({
  query,
  columns,
  rowSelection,
  setRowSelection,
  bulkActions,
  onFetchFullData,
  onAdd,
  onAddLabel,
  toolbar,
  onImportCSV,
  onImportJSON,
  extraActions,
}: GradeLevelsListContainerProps) {
  const {
    page,
    setPage,
    sortBy,
    setSortBy,
    sortOrder,
    setSortOrder,
    limit,
    setLimit,
    search,
    setSearch,
  } = useGradeLevelsSearchParams()
  const { data, isLoading } = query

  const [columnVisibility, setColumnVisibility] = React.useState({})

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden h-full">
      <div className="flex-1 overflow-y-auto">
        <DataTable
          columns={columns}
          data={data?.data || []}
          pageIndex={(page ?? 1) - 1}
          pageSize={limit || 10}
          pageCount={data?.total_pages || 0}
          canNextPage={(page ?? 1) < (data?.total_pages || 0)}
          canPreviousPage={(page ?? 1) > 1}
          fetchNextPage={() => setPage((page ?? 1) + 1)}
          fetchPreviousPage={() => setPage((page ?? 1) - 1)}
          sorting={[
            { id: sortBy ?? 'grade_number', desc: sortOrder === 'desc' },
          ]}
          onSortingChange={(updaterOrValue) => {
            const nextSorting =
              typeof updaterOrValue === 'function'
                ? updaterOrValue([
                    {
                      id: sortBy ?? 'grade_number',
                      desc: sortOrder === 'desc',
                    },
                  ])
                : updaterOrValue
            const firstSort = nextSorting[0]
            if (firstSort) {
              setSortBy(firstSort.id)
              setSortOrder(firstSort.desc ? 'desc' : 'asc')
            }
          }}
          columnVisibility={columnVisibility}
          onColumnVisibilityChange={setColumnVisibility}
          rowSelection={rowSelection}
          onRowSelectionChange={setRowSelection}
          isLoading={isLoading}
          onPageSizeChange={setLimit}
          onPageIndexChange={(index: number) => setPage(index + 1)}
          showDefaultToolbar={true}
          toolbar={toolbar}
          onFetchFullData={onFetchFullData}
          onImportCSV={onImportCSV}
          onImportJSON={onImportJSON}
          onAdd={onAdd}
          onAddLabel={onAddLabel}
          bulkActions={bulkActions}
          enableSelection
          enablePinning
          search={search ?? ''}
          onSearchChange={setSearch}
          searchPlaceholder="Search grade levels..."
          extraActions={extraActions}
        />
      </div>
    </div>
  )
}
