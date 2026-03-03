import * as React from 'react'
import type { SubjectResponse } from '@/lib/api/types.gen'
import type { ClassAssignmentRow } from './class-assignments-table-columns'
import type { UseQueryResult } from '@tanstack/react-query'
import type {
  DataTableColumnDef,
  DataTableToolbarContext,
} from '@/components/data-table'
import { DataTable } from '@/components/data-table'

interface ClassAssignmentsListContainerProps {
  query: UseQueryResult<Array<SubjectResponse>, Error>
  columns: Array<DataTableColumnDef<ClassAssignmentRow, unknown>>
  data: Array<ClassAssignmentRow>
  onAdd?: () => void
  onAddLabel?: string
  toolbar?: (
    context: DataTableToolbarContext<ClassAssignmentRow>,
  ) => React.ReactNode
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  extraActions?: React.ReactNode
}

export function ClassAssignmentsListContainer({
  query,
  columns,
  data,
  onAdd,
  onAddLabel,
  toolbar,
  onImportCSV,
  onImportJSON,
  extraActions,
}: ClassAssignmentsListContainerProps) {
  const [page, setPage] = React.useState(1)
  const [pageSize, setPageSize] = React.useState(10)
  const [sorting, setSorting] = React.useState<
    Array<{ id: string; desc: boolean }>
  >([])
  const [search, setSearch] = React.useState('')
  const { isLoading } = query

  const [columnVisibility, setColumnVisibility] = React.useState({})

  const filteredData = React.useMemo(() => {
    if (!search) return data
    const s = search.toLowerCase()
    return data.filter(
      (item) =>
        item.subjectName?.toLowerCase().includes(s) ||
        item.teacherName?.toLowerCase().includes(s) ||
        item.className?.toLowerCase().includes(s),
    )
  }, [data, search])

  const pageCount = Math.ceil(filteredData.length / pageSize)

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden h-full">
      <div className="flex-1 overflow-y-auto">
        <DataTable
          columns={columns}
          data={filteredData}
          pageIndex={page - 1}
          pageSize={pageSize}
          pageCount={pageCount}
          canNextPage={page < pageCount}
          canPreviousPage={page > 1}
          fetchNextPage={() => setPage(page + 1)}
          fetchPreviousPage={() => setPage(page - 1)}
          sorting={sorting}
          onSortingChange={(updaterOrValue) => {
            const nextSorting =
              typeof updaterOrValue === 'function'
                ? updaterOrValue(sorting)
                : updaterOrValue
            setSorting(nextSorting)
          }}
          columnVisibility={columnVisibility}
          onColumnVisibilityChange={setColumnVisibility}
          isLoading={isLoading}
          onPageSizeChange={setPageSize}
          onPageIndexChange={(index: number) => setPage(index + 1)}
          showDefaultToolbar={true}
          toolbar={toolbar}
          onImportCSV={onImportCSV}
          onImportJSON={onImportJSON}
          onAdd={onAdd}
          onAddLabel={onAddLabel}
          enableSelection
          enablePinning
          search={search}
          onSearchChange={setSearch}
          searchPlaceholder="Search assignments..."
          extraActions={extraActions}
        />
      </div>
    </div>
  )
}
