import { useStudentsStore } from '../store'
import { StudentBoardView } from './student-board-view'
import type {
  ColumnDef,
  RowSelectionState,
  Updater,
} from '@tanstack/react-table'
import type { UseQueryResult } from '@tanstack/react-query'
import type {
  PaginatedStudentResponse,
  StudentResponse,
} from '@/lib/api/types.gen'
import { DataTable } from '@/components/ui/data-table'

import { Button } from '@/components/ui/button'

interface StudentListContainerProps {
  studentsQuery: UseQueryResult<PaginatedStudentResponse>
  columns: Array<ColumnDef<StudentResponse>>
  limit: number
  rowSelection: RowSelectionState
  setRowSelection: (updater: Updater<RowSelectionState>) => void
}

export function StudentListContainer({
  studentsQuery,
  columns,
  limit,
  rowSelection,
  setRowSelection,
}: StudentListContainerProps) {
  const { view, page, setPage, setStudentToEdit, setStudentToDelete } =
    useStudentsStore()

  const students = studentsQuery.data?.data ?? []
  const totalPages = studentsQuery.data?.total_pages ?? 0

  if (view === 'table') {
    return (
      <div className="px-8">
        <DataTable<StudentResponse, unknown>
          columns={columns}
          data={students}
          pageIndex={page - 1}
          pageSize={limit}
          pageCount={totalPages}
          canNextPage={page < totalPages}
          canPreviousPage={page > 1}
          fetchNextPage={() => setPage(page + 1)}
          fetchPreviousPage={() => setPage(page - 1)}
          rowSelection={rowSelection}
          onRowSelectionChange={setRowSelection}
          isLoading={studentsQuery.isFetching}
        />
      </div>
    )
  }

  return (
    <div className="px-8 py-4 space-y-4">
      <StudentBoardView
        students={students}
        isLoading={studentsQuery.isFetching}
        onEdit={(student) => setStudentToEdit(student)}
        onDelete={setStudentToDelete}
      />
      {totalPages > 1 && (
        <div className="mt-8 flex items-center justify-center gap-4">
          <Button
            variant="outline"
            size="sm"
            onClick={() => setPage(Math.max(1, page - 1))}
            disabled={page === 1 || studentsQuery.isLoading}
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
            disabled={page === totalPages || studentsQuery.isLoading}
          >
            Next
          </Button>
        </div>
      )}
    </div>
  )
}
