import * as React from 'react'
import { useAcademicYearsSearchParams } from '../search-params'
import type {
  AcademicYearResponse,
  PaginatedAcademicYearResponse,
} from '@/lib/api/types.gen'
import type { UseQueryResult } from '@tanstack/react-query'
import type {
  DataTableColumnDef,
  DataTableToolbarContext,
} from '@/components/data-table'
import type { Table } from '@tanstack/react-table'
import { DataTable } from '@/components/data-table'

interface AcademicYearsListContainerProps {
  query: UseQueryResult<PaginatedAcademicYearResponse, Error>
  columns: Array<DataTableColumnDef<AcademicYearResponse, unknown>>
  rowSelection: Record<string, boolean>
  setRowSelection: React.Dispatch<React.SetStateAction<Record<string, boolean>>>
  bulkActions?: (context: {
    selectedRows: Array<AcademicYearResponse>
    table: Table<AcademicYearResponse>
  }) => React.ReactNode
  onFetchFullData?: () => Promise<Array<AcademicYearResponse>>
  onAdd?: () => void
  onAddLabel?: string
  toolbar?: (
    context: DataTableToolbarContext<AcademicYearResponse>,
  ) => React.ReactNode
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  extraActions?: React.ReactNode
}

export function AcademicYearsListContainer({
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
}: AcademicYearsListContainerProps) {
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
  } = useAcademicYearsSearchParams()
  const { data, isLoading } = query

  const [columnVisibility, setColumnVisibility] = React.useState({})

  return (
    <div className="overflow-y-auto flex-1 h-full">
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
        sorting={[{ id: sortBy ?? 'year_start', desc: sortOrder === 'desc' }]}
        onSortingChange={(updaterOrValue) => {
          const nextSorting =
            typeof updaterOrValue === 'function'
              ? updaterOrValue([
                  { id: sortBy ?? 'year_start', desc: sortOrder === 'desc' },
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
        searchPlaceholder="Search academic years..."
        extraActions={extraActions}
      />
    </div>
  )
}
