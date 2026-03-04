import * as React from 'react'
import type { UseQueryResult } from '@tanstack/react-query'
import type { CurriculumStandardResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { DataTable } from '@/components/data-table'
import { Box } from '@/components/primitives'

interface CurriculumListContainerProps {
  query: UseQueryResult<Array<CurriculumStandardResponse>, Error>
  columns: Array<DataTableColumnDef<CurriculumStandardResponse>>
  search: string
}

export function CurriculumListContainer({
  query,
  columns,
  search,
}: CurriculumListContainerProps) {
  const filteredData = React.useMemo(() => {
    const data = query.data || []
    if (!search) return data
    const s = search.toLowerCase()
    return data.filter(
      (item) =>
        item.standard_code.toLowerCase().includes(s) ||
        item.version_name?.toLowerCase().includes(s) ||
        item.description?.toLowerCase().includes(s),
    )
  }, [query.data, search])

  return (
    <Box className="flex-1 overflow-hidden border rounded-xl bg-card">
      <DataTable
        columns={columns}
        data={filteredData}
        isLoading={query.isLoading}
        search={search}
        onSearchChange={() => {}} // Placeholder, as no search is implemented in this component
        pageIndex={0}
        pageSize={filteredData.length || 10} // Display all filtered data on one page for simplicity
        pageCount={1}
        canNextPage={false}
        canPreviousPage={false}
        fetchNextPage={() => {}}
        fetchPreviousPage={() => {}}
        searchPlaceholder="Filter standards..."
      />
    </Box>
  )
}
