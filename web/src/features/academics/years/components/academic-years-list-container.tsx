'use client'

import * as React from 'react'
import { useAcademicYearsStore } from '../store'
import type { ColumnDef, RowSelectionState } from '@tanstack/react-table'
import type { UseQueryResult } from '@tanstack/react-query'
import type {
  AcademicYearResponse,
  PaginatedAcademicYearResponse,
} from '@/lib/api'
import { DataTable } from '@/components/ui/data-table'

interface AcademicYearsListContainerProps {
  query: UseQueryResult<PaginatedAcademicYearResponse, unknown>
  columns: Array<ColumnDef<AcademicYearResponse>>
  rowSelection: RowSelectionState
  setRowSelection: React.Dispatch<React.SetStateAction<RowSelectionState>>
}

export function AcademicYearsListContainer({
  query,
  columns,
  rowSelection,
  setRowSelection,
}: AcademicYearsListContainerProps) {
  const { setPage } = useAcademicYearsStore()
  const { data, isLoading } = query

  const pageIndex = (data?.page ?? 1) - 1
  const pageSize = data?.limit ?? 10
  const pageCount = data?.total_pages ?? 0
  const canNextPage = (data?.page ?? 1) < (data?.total_pages ?? 0)
  const canPreviousPage = (data?.page ?? 1) > 1

  const fetchNextPage = () => setPage(pageIndex + 2)
  const fetchPreviousPage = () => setPage(pageIndex)

  return (
    <div className="p-4">
      <div className="rounded-md border">
        <DataTable
          columns={columns}
          data={data?.data ?? []}
          isLoading={isLoading}
          rowSelection={rowSelection}
          onRowSelectionChange={setRowSelection}
          pageIndex={pageIndex}
          pageSize={pageSize}
          pageCount={pageCount}
          canNextPage={canNextPage}
          canPreviousPage={canPreviousPage}
          fetchNextPage={fetchNextPage}
          fetchPreviousPage={fetchPreviousPage}
        />
      </div>
    </div>
  )
}
