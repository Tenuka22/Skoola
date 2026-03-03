import { useStaffSearchParams } from '../search-params'
import { StaffGridView } from './staff-grid-view'
import type {
  ColumnDef,
  RowSelectionState,
  Updater,
} from '@tanstack/react-table'
import type { UseQueryResult } from '@tanstack/react-query'
import type { StaffResponse } from '@/lib/api/types.gen'
import { DataTable } from '@/components/ui/data-table'
import { Tabs, TabsContent } from '@/components/ui/tabs'
import { Stack } from '@/components/primitives'

interface StaffListContainerProps {
  staffQuery: UseQueryResult<{
    data?: Array<StaffResponse>
    total?: number
    total_pages?: number
  }>
  columns: Array<ColumnDef<StaffResponse>>
  rowSelection: RowSelectionState
  setRowSelection: (updater: Updater<RowSelectionState>) => void
  setStaffToEdit: (staff: StaffResponse | null) => void
  setStaffToDelete: (id: string | null) => void
  setIsCreateStaffOpen: (open: boolean) => void
}

export function StaffListContainer({
  staffQuery,
  columns,
  rowSelection,
  setRowSelection,
  setStaffToEdit,
  setStaffToDelete,
  setIsCreateStaffOpen,
}: StaffListContainerProps) {
  const { view, page, setPage, sortBy, setSortBy, sortOrder, setSortOrder } =
    useStaffSearchParams()

  const staffMembers = staffQuery.data?.data || []
  const totalPages = staffQuery.data?.total_pages || 0

  return (
    <Tabs value={view ?? 'table'} defaultValue="table">
      <TabsContent value="table" className="flex w-full">
        <div className="overflow-y-auto w-0 flex-1">
          <DataTable<StaffResponse, unknown>
            columns={columns}
            data={staffMembers}
            pageIndex={(page ?? 1) - 1}
            pageSize={10}
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
            rowSelection={rowSelection}
            onRowSelectionChange={setRowSelection}
            isLoading={staffQuery.isFetching}
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
