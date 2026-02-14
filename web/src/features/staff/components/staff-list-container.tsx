import { useStaffStore } from '../store'
import { StaffBoardView } from './staff-board-view'
import type {
  ColumnDef,
  RowSelectionState,
  Updater,
} from '@tanstack/react-table'
import type { UseQueryResult } from '@tanstack/react-query'
import type { StaffResponse } from '@/lib/api/types.gen'
import { DataTable } from '@/components/ui/data-table'

import { Button } from '@/components/ui/button'

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

  if (view === 'table') {
    return (
      <div className="px-8">
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
    )
  }

  return (
    <div className="px-8 py-4 space-y-4">
      <StaffBoardView
        staff={staffMembers}
        isLoading={staffQuery.isFetching}
        onEdit={(staff) => setStaffToEdit(staff)}
        onDelete={setStaffToDelete}
      />
      {totalPages > 1 && (
        <div className="mt-8 flex items-center justify-center gap-4">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage(Math.max(1, page - 1))}
            disabled={page === 1 || staffQuery.isLoading}
          >
            Previous
          </Button>
          <div className="text-sm font-medium">
            Page {page} of {totalPages}
          </div>
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage(Math.min(totalPages, page + 1))}
            disabled={page === totalPages || staffQuery.isLoading}
          >
            Next
          </Button>
        </div>
      )}
    </div>
  )
}
