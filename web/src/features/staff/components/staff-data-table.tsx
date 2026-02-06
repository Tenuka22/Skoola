import { useQuery } from '@tanstack/react-query'
import type { ColumnDef } from '@tanstack/react-table'
import type { StaffResponse } from '@/lib/api/types.gen'
import { DataTable } from '@/components/ui/data-table'
import { getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options } from '@/lib/api/@tanstack/react-query.gen'

type StaffDataTableProps = object
// No longer needs to define props for rows and columns as they are fetched internally

export const StaffDataTable = (_props: StaffDataTableProps) => {
  const columns: Array<ColumnDef<StaffResponse>> = [
    { accessorKey: 'name', header: 'Name' },
    { accessorKey: 'email', header: 'Email' },
    { accessorKey: 'employee_id', header: 'Employee ID' },
    { accessorKey: 'staff_type', header: 'Staff Type' },
    { accessorKey: 'employment_status', header: 'Employment Status' },
    // TODO: Add more columns as needed, e.g., for actions
  ]

  const { data, isLoading, isError } = useQuery(
    getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options(),
  )

  const staffData = data || []

  if (isLoading) {
    return <div className="p-4">Loading staff data...</div>
  }

  if (isError) {
    return <div className="p-4 text-red-500">Error loading staff data.</div>
  }

  // Assuming the API returns total_staff and current offset/limit

  return (
    <div>
      <h2 className="text-xl font-semibold mb-4">Staff Directory</h2>
      <DataTable
        columns={columns}
        data={staffData}
        pageIndex={0}
        pageSize={staffData.length}
        pageCount={1}
        canNextPage={false}
        canPreviousPage={false}
        fetchNextPage={() => {}}
        fetchPreviousPage={() => {}}
      />
    </div>
  )
}
