import type {
  ColumnDef,
  RowSelectionState,
  Updater,
} from '@tanstack/react-table'
import type { UseQueryResult } from '@tanstack/react-query'
import { useStaffStore } from '../store'
import { DataTable } from '@/components/ui/data-table'
import { StaffCard } from './staff-card'
import type { StaffResponse } from '@/lib/api/types.gen'

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
    <div className="px-8 pb-8">
      {staffQuery.isLoading ? (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 6 }).map((_, i) => (
            <div
              key={i}
              className="h-64 animate-pulse rounded-[2.5rem] bg-muted/50"
            />
          ))}
        </div>
      ) : staffMembers.length > 0 ? (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
          {staffMembers.map((staff) => (
            <StaffCard
              key={staff.id}
              staff={staff}
              onEdit={setStaffToEdit}
              onDelete={setStaffToDelete}
            />
          ))}
        </div>
      ) : (
        <div className="flex h-64 flex-col items-center justify-center rounded-[2.5rem] border-2 border-dashed">
          <p className="text-muted-foreground">No staff members found.</p>
        </div>
      )}

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
