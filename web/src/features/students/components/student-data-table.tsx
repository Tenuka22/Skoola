import React from 'react'
import { useInfiniteQuery } from '@tanstack/react-query'
import type { ColumnDef } from '@tanstack/react-table'
import type { StudentResponse } from '@/lib/api/types.gen'
import { DataTable } from '@/components/ui/data-table'
import { getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1InfiniteOptions } from '@/lib/api/@tanstack/react-query.gen'

type StudentDataTableProps = object // No longer needs to define props for rows and columns as they are fetched internally

export const StudentDataTable = (_props: StudentDataTableProps) => {
  const columns: Array<ColumnDef<StudentResponse>> = [
    { accessorKey: 'name_english', header: 'Name' },
    { accessorKey: 'admission_number', header: 'Admission No.' },
    { accessorKey: 'gender', header: 'Gender' },
    { accessorKey: 'status', header: 'Status' },
    // TODO: Add more columns as needed, e.g., for current class, actions
  ]

  const {
    data,
    fetchNextPage,
    fetchPreviousPage,
    hasNextPage,
    hasPreviousPage,
    isLoading,
    isError,
  } = useInfiniteQuery({
    ...getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1InfiniteOptions({
      query: {
        limit: 10n, // Adjust limit as needed
      },
    }),
    initialPageParam: 0n,
    getNextPageParam: (lastPage, _allPages, lastPageParam) => {
      if ((lastPage.students?.length || 0) < 10) {
        return undefined
      }
      return (lastPageParam as bigint) + 10n
    },
  })

  const flatData = React.useMemo(
    () => data?.pages?.flatMap((page) => page.students || []) || [],
    [data],
  )

  if (isLoading) {
    return <div className="p-4">Loading student data...</div>
  }

  if (isError) {
    return <div className="p-4 text-red-500">Error loading student data.</div>
  }

  // Assuming the API returns total_students and current offset/limit
  const totalStudents =
    (data &&
      data.pages &&
      data.pages.length > 0 &&
      data.pages[0].total_students) ||
    0
  const limit =
    (data && data.pages && data.pages.length > 0 && data.pages[0].limit) || 10
  const offset =
    (data &&
      data.pages &&
      data.pages.length > 0 &&
      data.pages[data.pages.length - 1].offset) ||
    0

  return (
    <div>
      <h2 className="text-xl font-semibold mb-4">Student Directory</h2>
      <DataTable
        columns={columns}
        data={flatData}
        pageIndex={Math.floor(Number(offset) / Number(limit))}
        pageSize={Number(limit)}
        pageCount={Math.ceil(Number(totalStudents) / Number(limit))}
        canNextPage={hasNextPage}
        canPreviousPage={hasPreviousPage}
        fetchNextPage={() => fetchNextPage()}
        fetchPreviousPage={() => fetchPreviousPage()}
      />
    </div>
  )
}
