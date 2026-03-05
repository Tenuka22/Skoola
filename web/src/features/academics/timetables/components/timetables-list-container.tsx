import * as React from 'react'
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
  onFetchFullData?: () => Promise<Array<TimetableEntryRow>>
  toolbar?: (
    context: DataTableToolbarContext<TimetableEntryRow>,
  ) => React.ReactNode
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  onAdd?: () => void
  onAddLabel?: string
  extraActions?: React.ReactNode
  search?: string
  onSearchChange?: (value: string) => void
}

export function TimetablesListContainer({
  query,
  columns,
  data,
  onFetchFullData,
  toolbar,
  onImportCSV,
  onImportJSON,
  onAdd,
  onAddLabel,
  extraActions,
  search: externalSearch,
  onSearchChange: externalOnSearchChange,
}: TimetablesListContainerProps) {
  const [sorting, setSorting] = React.useState<
    Array<{ id: string; desc: boolean }>
  >([])
  const [internalSearch, setInternalSearch] = React.useState('')
  const search = externalSearch ?? internalSearch
  const onSearchChange = externalOnSearchChange ?? setInternalSearch

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
          onSearchChange={onSearchChange}
          searchPlaceholder="Search timetable..."
          extraActions={extraActions}
        />
      </div>
    </div>
  )
}
