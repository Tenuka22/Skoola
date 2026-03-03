import * as React from 'react'
import { useStaffSearchParams } from '../search-params'
import { StaffGridView } from './staff-grid-view'
import type { RowSelectionState, Table, Updater } from '@tanstack/react-table'
import type { UseQueryResult } from '@tanstack/react-query'
import type { StaffResponse } from '@/lib/api/types.gen'
import type {
  DataTableColumnDef,
  DataTableFacetedFilter,
  DataTableToolbarContext,
} from '@/components/data-table'
import { DataTable } from '@/components/data-table'
import { Tabs, TabsContent } from '@/components/ui/tabs'
import { Stack } from '@/components/primitives'

interface StaffListContainerProps {
  staffQuery: UseQueryResult<{
    data?: Array<StaffResponse>
    total?: number
    total_pages?: number
  }>
  columns: Array<DataTableColumnDef<StaffResponse>>
  rowSelection: RowSelectionState
  setRowSelection: (updater: Updater<RowSelectionState>) => void
  setStaffToEdit: (staff: StaffResponse | null) => void
  setStaffToDelete: (id: string | null) => void
  setIsCreateStaffOpen: (open: boolean) => void
  bulkActions?: (context: {
    selectedRows: Array<StaffResponse>
    table: Table<StaffResponse>
  }) => React.ReactNode
  facetedFilters?: Array<DataTableFacetedFilter>
  onFetchFullData?: () => Promise<Array<StaffResponse>>
  onAdd?: () => void
  onAddLabel?: string
  toolbar?: (context: DataTableToolbarContext<StaffResponse>) => React.ReactNode
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  extraActions?: React.ReactNode
}

export function StaffListContainer({
  staffQuery,
  columns,
  rowSelection,
  setRowSelection,
  setStaffToEdit,
  setStaffToDelete,
  setIsCreateStaffOpen,
  bulkActions,
  facetedFilters,
  onFetchFullData,
  onAdd,
  onAddLabel,
  toolbar,
  onImportCSV,
  onImportJSON,
  extraActions,
}: StaffListContainerProps) {
  const {
    view,
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
  } = useStaffSearchParams()

  const staffMembers = staffQuery.data?.data || []
  const totalPages = staffQuery.data?.total_pages || 0

  const [columnVisibility, setColumnVisibility] = React.useState({})

  return (
    <Tabs value={view ?? 'table'} defaultValue="table">
      <TabsContent value="table" className="flex w-full">
        <div className="overflow-y-auto w-0 flex-1">
          <DataTable<StaffResponse>
            columns={columns}
            data={staffMembers}
            pageIndex={(page ?? 1) - 1}
            pageSize={limit || 10}
            pageCount={totalPages}
            canNextPage={(page ?? 1) < totalPages}
            canPreviousPage={(page ?? 1) > 1}
            fetchNextPage={() => setPage((page ?? 1) + 1)}
            fetchPreviousPage={() => setPage((page ?? 1) - 1)}
            sorting={[
              { id: sortBy ?? 'created_at', desc: sortOrder === 'desc' },
            ]}
            onSortingChange={(updaterOrValue) => {
              const newSorting =
                typeof updaterOrValue === 'function'
                  ? updaterOrValue([
                      {
                        id: sortBy ?? 'created_at',
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
            columnVisibility={columnVisibility}
            onColumnVisibilityChange={setColumnVisibility}
            rowSelection={rowSelection}
            onRowSelectionChange={setRowSelection}
            isLoading={staffQuery.isFetching}
            onPageSizeChange={setLimit}
            onPageIndexChange={(index: number) => setPage(index + 1)}
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
            onSearchChange={setSearch}
            searchPlaceholder="Search staff..."
            extraActions={extraActions}
          />
        </div>
      </TabsContent>

      <TabsContent value="grid">
        <Stack gap={4}>
          <StaffGridView
            staff={staffMembers}
            isLoading={staffQuery.isFetching}
            onEdit={(staff) => setStaffToEdit(staff)}
            onDelete={setStaffToDelete}
            setIsCreateStaffOpen={setIsCreateStaffOpen}
          />
        </Stack>
      </TabsContent>
    </Tabs>
  )
}
