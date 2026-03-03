import * as React from 'react'
import { TimetablesVisualView } from './timetables-visual-view'
import type { TimetableResponse } from '@/lib/api/types.gen'
import type { TimetableEntryRow } from './timetables-table-columns'
import type { UseQueryResult } from '@tanstack/react-query'
import type {
  DataTableColumnDef,
  DataTableToolbarContext,
} from '@/components/data-table'
import { DataTable } from '@/components/data-table'

interface TimetablesListContainerProps {
  query: UseQueryResult<Array<TimetableResponse>, Error>
  columns: Array<DataTableColumnDef<TimetableEntryRow, unknown>>
  data: Array<TimetableEntryRow>
  isGridView: boolean
  viewMode: string
  onEdit: (entry: TimetableResponse | null) => void
  onFetchFullData?: () => Promise<Array<TimetableEntryRow>>
  onAdd?: () => void
  onAddLabel?: string
  toolbar?: (
    context: DataTableToolbarContext<TimetableEntryRow>,
  ) => React.ReactNode
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  extraActions?: React.ReactNode
}

export function TimetablesListContainer({
  query,
  columns,
  data,
  isGridView,
  viewMode,
  onEdit,
  onFetchFullData,
  onAdd,
  onAddLabel,
  toolbar,
  onImportCSV,
  onImportJSON,
  extraActions,
}: TimetablesListContainerProps) {
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

  if (isGridView) {
    return (
      <TimetablesVisualView
        data={filteredData}
        viewMode={viewMode}
        setTimetableEntryToEdit={onEdit}
      />
    )
  }

  return (
    <div className="relative flex-1 flex flex-col overflow-hidden h-full">
      <div className="flex-1 overflow-y-auto">
        <DataTable
          columns={columns}
          data={filteredData}
          pageIndex={0}
          pageSize={filteredData.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
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
          onPageSizeChange={() => {}}
          onPageIndexChange={() => {}}
          showDefaultToolbar={true}
          toolbar={toolbar}
          onFetchFullData={onFetchFullData}
          onImportCSV={onImportCSV}
          onImportJSON={onImportJSON}
          onAdd={onAdd}
          onAddLabel={onAddLabel}
          enableSelection
          enablePinning
          search={search}
          onSearchChange={setSearch}
          searchPlaceholder="Search timetable..."
          extraActions={extraActions}
        />
      </div>
    </div>
  )
}
