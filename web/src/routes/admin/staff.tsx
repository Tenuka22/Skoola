import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import { StaffHeader } from '../../features/staff/components/staff-header'
import { StaffToolbar } from '../../features/staff/components/staff-toolbar'
import { StaffFilters } from '../../features/staff/components/staff-filters'
import { StaffListContainer } from '../../features/staff/components/staff-list-container'
import { getStaffColumns } from '../../features/staff/components/staff-table-columns'
import { StaffAddDialog } from '../../features/staff/components/staff-add-dialog'
import { StaffDeleteDialog } from '../../features/staff/components/staff-delete-dialog'
import { StaffEditDialog } from '../../features/staff/components/staff-edit-dialog'
import { useStaffStore } from '../../features/staff/store'
import { handleExportCSV } from '../../lib/export'
import { authClient } from '../../lib/clients'
import {
  isEmploymentStatus,
  isStaffType,
} from '../../features/staff/utils/staff-guards'
import {
  deleteStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation,
  getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options,
  getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857QueryKey,
  postStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Mutation,
  putStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const Route = createFileRoute('/admin/staff')({
  component: StaffPage,
})

function StaffPage() {
  const store = useStaffStore()
  const { search, setDebouncedSearch } = store

  const limit = 10

  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearch(search)
    }, 400)
    return () => clearTimeout(handler)
  }, [search, setDebouncedSearch])

  const {
    page,
    staffTypeFilter,
    employmentStatusFilter,
    debouncedSearch,
    setStaffToDelete,
    setStaffToEdit,
    setIsCreateStaffOpen,
  } = store

  const staffQuery = useQuery({
    ...getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Options({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        staff_type:
          staffTypeFilter === 'all'
            ? undefined
            : isStaffType(staffTypeFilter)
              ? staffTypeFilter
              : undefined,
        employment_status:
          employmentStatusFilter === 'all'
            ? undefined
            : isEmploymentStatus(employmentStatusFilter)
              ? employmentStatusFilter
              : undefined,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const queryClient = useQueryClient()
  const invalidateStaff = () => {
    queryClient.invalidateQueries({
      queryKey: getStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857QueryKey(),
    })
  }

  const deleteStaff = useMutation({
    ...deleteStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Staff member deleted successfully.`)
      invalidateStaff()
      setStaffToDelete(null)
    },
    onError: (error) => {
      toast.error(`Failed to delete staff: ${error.message || 'Unknown error'}`)
    },
  })

  const createStaff = useMutation({
    ...postStaffDb2Ddf96Bd86Cfcd0342B203Ba78A857Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Staff member created successfully.`)
      invalidateStaff()
      setIsCreateStaffOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to create staff: ${error.message || 'Unknown error'}`)
    },
  })

  const updateStaff = useMutation({
    ...putStaffA2C17Fd0026652C749Fc88Fc4Fd7Fd58Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Staff member updated successfully.`)
      invalidateStaff()
      setStaffToEdit(null)
    },
    onError: (error) => {
      toast.error(`Failed to update staff: ${error.message || 'Unknown error'}`)
    },
  })

  const columns = getStaffColumns({
    onEdit: store.setStaffToEdit,
    onDelete: store.setStaffToDelete,
  })

  const [rowSelection, setRowSelection] = React.useState({})

  return (
    <div className="flex h-full flex-col bg-background">
      <StaffHeader />
      <StaffToolbar
        onExport={() =>
          handleExportCSV(staffQuery.data?.data || [], 'staff_export.csv', [
            { header: 'ID', accessor: 'employee_id' },
            { header: 'Name', accessor: 'name' },
            { header: 'Email', accessor: 'email' },
            { header: 'Type', accessor: 'staff_type' },
            { header: 'Status', accessor: 'employment_status' },
          ])
        }
      />
      <StaffFilters />
      <StaffListContainer
        staffQuery={staffQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <StaffDeleteDialog
        staffToDeleteId={store.staffToDelete}
        setStaffToDeleteId={store.setStaffToDelete}
        onDeleteConfirm={(id) => deleteStaff.mutate({ path: { staff_id: id } })}
      />

      <StaffAddDialog
        isAddOpen={store.isCreateStaffOpen}
        setIsAddOpen={store.setIsCreateStaffOpen}
        onAddConfirm={(values) => createStaff.mutate({ body: values })}
        isAdding={createStaff.isPending}
      />

      <StaffEditDialog
        staffToEdit={store.staffToEdit}
        setStaffToEdit={store.setStaffToEdit}
        onEditConfirm={(values) =>
          store.staffToEdit &&
          updateStaff.mutate({
            path: { staff_id: store.staffToEdit.id },
            body: values,
          })
        }
        isEditing={updateStaff.isPending}
      />
    </div>
  )
}
