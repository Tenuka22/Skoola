import * as React from 'react'
import { useStaffSearchParams } from '../search-params'
import type { Staff } from '../types'
import type {
  DataTableColumnDef,
  DataTableFacetedFilter,
  DataTableToolbarContext,
} from '@/components/data-table'
import type { ColumnFiltersState, Table } from '@tanstack/react-table'
import { DataTable } from '@/components/data-table'

interface StaffListContainerProps {
  staffQuery: {
    data?: {
      data: Array<Staff>
      total: number
    }
    isLoading: boolean
    isFetching: boolean
  }
  limit: number
  columns: Array<DataTableColumnDef<Staff>>
  rowSelection: Record<string, boolean>
  setRowSelection: (
    selection:
      | Record<string, boolean>
      | ((prev: Record<string, boolean>) => Record<string, boolean>),
  ) => void
  contextMenuItems?: (row: Staff) => React.ReactNode
  toolbar?: (context: DataTableToolbarContext<Staff>) => React.ReactNode
  facetedFilters?: Array<DataTableFacetedFilter>
  onFetchFullData?: () => Promise<Array<Staff>>
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  onAdd?: () => void
  onAddLabel?: string
  bulkActions?: (context: {
    selectedRows: Array<Staff>
    table: Table<Staff>
  }) => React.ReactNode
  extraActions?: React.ReactNode
  onPageChange?: (page: number) => void
  onLimitChange?: (limit: number) => void
  onSortChange?: (sort: Array<{ id: string; desc: boolean }>) => void
  onSearchChange?: (search: string) => void
}

export function StaffListContainer({
  staffQuery,
  limit,
  columns,
  rowSelection,
  setRowSelection,
  contextMenuItems,
  toolbar,
  facetedFilters,
  onFetchFullData,
  onImportCSV,
  onImportJSON,
  onAdd,
  onAddLabel,
  bulkActions,
  extraActions,
  onPageChange,
  onLimitChange,
  onSortChange,
  onSearchChange,
}: StaffListContainerProps) {
  const {
    page,
    sort,
    search,
  } = useStaffSearchParams()

  const [columnVisibility, setColumnVisibility] = React.useState({})
  const columnFilters: ColumnFiltersState = []

  return (
    <div className="flex w-full h-full">
      <div className="overflow-y-auto w-0 flex-1 h-full">
        <DataTable
          columns={columns}
          data={staffQuery.data?.data || []}
          pageIndex={(page || 1) - 1}
          pageSize={limit}
          pageCount={Math.ceil((staffQuery.data?.total || 0) / limit)}
          canPreviousPage={(page || 1) > 1}
          canNextPage={
            (page || 1) < Math.ceil((staffQuery.data?.total || 0) / limit)
          }
          fetchPreviousPage={() => onPageChange?.((page || 1) - 1)}
          fetchNextPage={() => onPageChange?.((page || 1) + 1)}
          sorting={sort ?? []}
          onSortingChange={(updaterOrValue) => {
            const nextSorting =
              typeof updaterOrValue === 'function'
                ? updaterOrValue(sort ?? [])
                : updaterOrValue
            onSortChange?.(nextSorting)
          }}
          columnVisibility={columnVisibility}
          onColumnVisibilityChange={setColumnVisibility}
          columnFilters={columnFilters}
          onColumnFiltersChange={() => {}}
          rowSelection={rowSelection}
          onRowSelectionChange={setRowSelection}
          isLoading={staffQuery.isFetching}
          onPageSizeChange={onLimitChange}
          onPageIndexChange={(index: number) => onPageChange?.(index + 1)}
          contextMenuItems={contextMenuItems}
          showDefaultToolbar={true}
          toolbar={toolbar}
          facetedFilters={facetedFilters}
          onFetchFullData={onFetchFullData}
          onImportCSV={onImportCSV}
          onImportJSON={onImportJSON}
          onAdd={onAdd}
          onAddLabel={onAddLabel}
          bulkActions={bulkActions}
          enableSelection
          enablePinning
          search={search ?? ''}
          onSearchChange={onSearchChange}
          searchPlaceholder="Search staff..."
          extraActions={extraActions}
        />
      </div>
    </div>
  )
}
