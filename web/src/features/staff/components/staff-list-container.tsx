import { useStaffStore } from '../store'
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
}

export function StaffListContainer({
  staffQuery,
  columns,
  rowSelection,
  setRowSelection,
}: StaffListContainerProps) {
  const { view, page, setPage, setStaffToEdit, setStaffToDelete } =
    useStaffStore()

  const staffMembers = staffQuery.data?.data || []
  const totalPages = staffQuery.data?.total_pages || 0

  return (
    <Tabs value={view} defaultValue="table">
      <TabsContent value="table" className="flex w-full">
        <div className="overflow-y-auto w-0 flex-1">
          <DataTable<StaffResponse, unknown>
            columns={columns}
            data={staffMembers}
            pageIndex={page - 1}
            pageSize={10}
            pageCount={totalPages}
            canNextPage={page < totalPages}
            canPreviousPage={page > 1}
            fetchNextPage={() => setPage(page + 1)}
            fetchPreviousPage={() => setPage(page - 1)}
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
          />
        </Stack>
      </TabsContent>
    </Tabs>
  )
}
