import * as React from 'react'
import { useStudentsSearchParams } from '../search-params'
import { StudentGridView } from './student-grid-view'
import type { RowSelectionState, Table, Updater } from '@tanstack/react-table'
import type { UseQueryResult } from '@tanstack/react-query'
import type {
  PaginatedStudentResponse,
  StudentResponse,
} from '@/lib/api/types.gen'
import type {
  DataTableColumnDef,
  DataTableFacetedFilter,
  DataTableToolbarContext,
} from '@/components/data-table'
import { DataTable } from '@/components/data-table'
import { Tabs, TabsContent } from '@/components/ui/tabs'

interface StudentListContainerProps {
  studentsQuery: UseQueryResult<PaginatedStudentResponse>
  columns: Array<DataTableColumnDef<StudentResponse>>
  limit: number
  rowSelection: RowSelectionState
  setRowSelection: (updater: Updater<RowSelectionState>) => void
  setStudentToEdit: (student: StudentResponse | null) => void
  setStudentToDelete: (id: string | null) => void
  bulkActions?: (context: {
    selectedRows: Array<StudentResponse>
    table: Table<StudentResponse>
  }) => React.ReactNode
  facetedFilters?: Array<DataTableFacetedFilter>
  onFetchFullData?: () => Promise<Array<StudentResponse>>
  onAdd?: () => void
  onAddLabel?: string
  toolbar?: (
    context: DataTableToolbarContext<StudentResponse>,
  ) => React.ReactNode
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  extraActions?: React.ReactNode
}

export function StudentListContainer({
  studentsQuery,
  columns,
  limit: _limit,
  rowSelection,
  setRowSelection,
  setStudentToEdit,
  setStudentToDelete,
  bulkActions,
  facetedFilters,
  onFetchFullData,
  onAdd,
  onAddLabel,
  toolbar,
  onImportCSV,
  onImportJSON,
  extraActions,
}: StudentListContainerProps) {
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
  } = useStudentsSearchParams()

  const students = studentsQuery.data?.data ?? []
  const totalPages = studentsQuery.data?.total_pages ?? 0

  const [columnVisibility, setColumnVisibility] = React.useState({})

  return (
    <Tabs value={view ?? 'table'} defaultValue="table">
      <TabsContent value="table" className="flex w-full">
        <div className="overflow-y-auto w-0 flex-1">
          <DataTable<StudentResponse>
            columns={columns}
            data={students}
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
            isLoading={studentsQuery.isFetching}
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
            searchPlaceholder="Search students..."
            extraActions={extraActions}
          />
        </div>
      </TabsContent>

      <TabsContent value="grid">
        <StudentGridView
          students={students}
          isLoading={studentsQuery.isFetching}
          onEdit={(student) => setStudentToEdit(student)}
          onDelete={setStudentToDelete}
        />
      </TabsContent>
    </Tabs>
  )
}
