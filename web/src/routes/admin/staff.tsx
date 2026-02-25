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
import { StaffBulkActionsToolbar } from '../../features/staff/components/staff-bulk-actions-toolbar'
import { StaffBulkEditDialog } from '../../features/staff/components/staff-bulk-edit-dialog'
import type { BulkEditStaffFormValues } from '@/features/staff/schemas'
import { StaffPhotoUploadDialog } from '@/features/staff/components/staff-photo-upload-dialog'
import { StaffAssignClassDialog } from '@/features/staff/components/staff-assign-class-dialog'
import { StaffAssignSubjectDialog } from '@/features/staff/components/staff-assign-subject-dialog'
import { StaffWorkloadDialog } from '@/features/staff/components/staff-workload-dialog'
import { StaffAttendanceDialog } from '@/features/staff/components/staff-attendance-dialog'
import { StaffLeaveDialog } from '@/features/staff/components/staff-leave-dialog'
import { StaffPermissionSetsDialog } from '@/features/staff/components/staff-permission-sets-dialog'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import {
  assignClassToTeacherMutation,
  assignSubjectToTeacherMutation,
  bulkDeleteStaffMutation,
  bulkUpdateStaffMutation,
  createStaffMutation,
  deleteStaffMutation,
  getAllStaffOptions,
  getAllStaffQueryKey,
  updateStaffMutation,
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
    isBulkDeleteOpen,
    setIsBulkDeleteOpen,
    isBulkEditOpen,
    setIsBulkEditOpen,
    isUploadPhotoOpen,
    setIsUploadPhotoOpen,
    staffToUploadPhotoFor,
    isAssignClassOpen,
    setIsAssignClassOpen,
    staffToAssignClassFor,
    isAssignSubjectOpen,
    setIsAssignSubjectOpen,
    staffToAssignSubjectFor,
    isWorkloadOpen,
    setIsWorkloadOpen,
    staffToViewWorkloadFor,
    isAttendanceOpen,
    setIsAttendanceOpen,
    staffToManageAttendanceFor,
    isLeavesOpen,
    setIsLeavesOpen,
    staffToManageLeavesFor,
    isPermissionSetsOpen,
    setIsPermissionSetsOpen,
    staffToManagePermissionSetsFor,
  } = store

  const staffQuery = useQuery({
    ...getAllStaffOptions({
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
      queryKey: getAllStaffQueryKey(),
    })
  }

  const deleteStaff = useMutation({
    ...deleteStaffMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Staff member deleted successfully.`)
      invalidateStaff()
      store.setStaffToDelete(null)
    },
    onError: (error) => {
      toast.error(`Failed to delete staff: ${error.message || 'Unknown error'}`)
    },
  })

  const createStaff = useMutation({
    ...createStaffMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Staff member created successfully.`)
      invalidateStaff()
      store.setIsCreateStaffOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to create staff: ${error.message || 'Unknown error'}`)
    },
  })

  const updateStaff = useMutation({
    ...updateStaffMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Staff member updated successfully.`)
      invalidateStaff()
      store.setStaffToEdit(null)
    },
    onError: (error) => {
      toast.error(`Failed to update staff: ${error.message || 'Unknown error'}`)
    },
  })

  const bulkDeleteStaff = useMutation({
    ...bulkDeleteStaffMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables.body?.staff_ids?.length ?? 0
      toast.success(`Successfully deleted ${count} staff members.`)
      invalidateStaff()
      setIsBulkDeleteOpen(false)
      setRowSelection({})
    },
    onError: (error) => {
      toast.error(
        `Failed to delete staff members: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const bulkUpdateStaff = useMutation({
    ...bulkUpdateStaffMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables.body?.staff_ids?.length ?? 0
      toast.success(`Successfully updated ${count} staff members.`)
      invalidateStaff()
      setIsBulkEditOpen(false)
      setRowSelection({})
    },
    onError: (error) => {
      toast.error(
        `Failed to update staff members: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const assignClass = useMutation({
    ...assignClassToTeacherMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Class assigned successfully.')
      setIsAssignClassOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to assign class: ${error.message || 'Unknown error'}`)
    },
  })

  const assignSubject = useMutation({
    ...assignSubjectToTeacherMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Subject assigned successfully.')
      setIsAssignSubjectOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to assign subject: ${error.message || 'Unknown error'}`)
    },
  })

  const columns = getStaffColumns({
    onEdit: store.setStaffToEdit,
    onDelete: store.setStaffToDelete,
    onUploadPhoto: store.setStaffToUploadPhotoFor,
    onAssignClass: store.setStaffToAssignClassFor,
    onAssignSubject: store.setStaffToAssignSubjectFor,
    onViewWorkload: store.setStaffToViewWorkloadFor,
    onManageAttendance: store.setStaffToManageAttendanceFor,
    onManageLeaves: store.setStaffToManageLeavesFor,
    onManagePermissions: store.setStaffToManagePermissionSetsFor,
  })

  const [rowSelection, setRowSelection] = React.useState<Record<string, boolean>>({})
  const selectedStaff = React.useMemo(() => {
    return new Set(
      Object.keys(rowSelection).filter((key) => rowSelection[key]),
    )
  }, [rowSelection])

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

      <StaffBulkActionsToolbar
        selectedStaff={selectedStaff}
        onBulkDelete={() => setIsBulkDeleteOpen(true)}
        onBulkEdit={() => setIsBulkEditOpen(true)}
      />

      <StaffDeleteDialog
        staffToDeleteId={store.staffToDelete}
        setStaffToDeleteId={store.setStaffToDelete}
        onDeleteConfirm={(id) => deleteStaff.mutate({ path: { staff_id: id } })}
      />

      <AlertDialog open={isBulkDeleteOpen} onOpenChange={setIsBulkDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete{' '}
              {selectedStaff.size} staff members and remove their data from our
              servers.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                bulkDeleteStaff.mutate({
                  body: { staff_ids: Array.from(selectedStaff) },
                })
              }}
            >
              Delete All
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <StaffBulkEditDialog
        open={isBulkEditOpen}
        onOpenChange={setIsBulkEditOpen}
        onConfirm={(data: BulkEditStaffFormValues) =>
          bulkUpdateStaff.mutate({
            body: { staff_ids: Array.from(selectedStaff), ...data },
          })
        }
        selectedCount={selectedStaff.size}
        isSubmitting={bulkUpdateStaff.isPending}
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

      <StaffPhotoUploadDialog
        staff={staffToUploadPhotoFor}
        open={isUploadPhotoOpen}
        onOpenChange={setIsUploadPhotoOpen}
      />

      <StaffAssignClassDialog
        staff={staffToAssignClassFor}
        open={isAssignClassOpen}
        onOpenChange={setIsAssignClassOpen}
        onConfirm={(staffId, data) =>
          assignClass.mutate({
            path: { teacher_id: staffId },
            body: data,
          })
        }
        isSubmitting={assignClass.isPending}
      />

      <StaffAssignSubjectDialog
        staff={staffToAssignSubjectFor}
        open={isAssignSubjectOpen}
        onOpenChange={setIsAssignSubjectOpen}
        onConfirm={(staffId, data) =>
          assignSubject.mutate({
            path: { teacher_id: staffId },
            body: data,
          })
        }
        isSubmitting={assignSubject.isPending}
      />

      <StaffWorkloadDialog
        staff={staffToViewWorkloadFor}
        open={isWorkloadOpen}
        onOpenChange={setIsWorkloadOpen}
      />

      <StaffAttendanceDialog
        staff={staffToManageAttendanceFor}
        open={isAttendanceOpen}
        onOpenChange={setIsAttendanceOpen}
      />

      <StaffLeaveDialog
        staff={staffToManageLeavesFor}
        open={isLeavesOpen}
        onOpenChange={setIsLeavesOpen}
      />

      <StaffPermissionSetsDialog
        staff={staffToManagePermissionSetsFor}
        open={isPermissionSetsOpen}
        onOpenChange={setIsPermissionSetsOpen}
      />
    </div>
  )
}
