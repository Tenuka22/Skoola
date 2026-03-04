import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { LayoutGridIcon, TableIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { StaffHeader } from '../../../features/staff/components/staff-header'
import { StaffFilters } from '../../../features/staff/components/staff-filters'
import { StaffListContainer } from '../../../features/staff/components/staff-list-container'
import { getStaffColumns } from '../../../features/staff/components/staff-table-columns'
import { StaffAddDialog } from '../../../features/staff/components/staff-add-dialog'
import { StaffDeleteDialog } from '../../../features/staff/components/staff-delete-dialog'
import { StaffEditDialog } from '../../../features/staff/components/staff-edit-dialog'
import {
  isEmploymentStatus,
  isStaffType,
} from '../../../features/staff/utils/staff-guards'
import { StaffBulkActionsToolbar } from '../../../features/staff/components/staff-bulk-actions-toolbar'
import { StaffBulkEditDialog } from '../../../features/staff/components/staff-bulk-edit-dialog'
import type { BulkEditStaffFormValues } from '@/features/staff/schemas'
import type { StaffResponse } from '@/lib/api/types.gen'
import type { DataTableFacetedFilter } from '@/components/data-table'
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
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Stack } from '@/components/primitives'
import {
  getAllStaffQueryOptions,
  useAssignClassToTeacher,
  useAssignSubjectToTeacher,
  useBulkDeleteStaff,
  useBulkUpdateStaff,
  useCreateStaff,
  useDeleteStaff,
  useUpdateStaff,
} from '@/features/staff/api'
import { useStaffSearchParams } from '@/features/staff/search-params'

export const Route = createFileRoute('/admin/staff/')({
  component: StaffPage,
})

function StaffPage() {
  const queryClient = useQueryClient()
  const {
    page,
    limit,
    search,
    staffTypeFilter,
    employmentStatusFilter,
    sortBy,
    sortOrder,
    view,
    setView,
  } = useStaffSearchParams()

  const [staffToDelete, setStaffToDelete] = React.useState<string | null>(null)
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isBulkEditOpen, setIsBulkEditOpen] = React.useState(false)
  const [isCreateStaffOpen, setIsCreateStaffOpen] = React.useState(false)
  const [staffToEdit, setStaffToEdit] = React.useState<StaffResponse | null>(
    null,
  )
  const [isUploadPhotoOpen, setIsUploadPhotoOpen] = React.useState(false)
  const [staffToUploadPhotoFor, setStaffToUploadPhotoFor] =
    React.useState<StaffResponse | null>(null)
  const [isAssignClassOpen, setIsAssignClassOpen] = React.useState(false)
  const [staffToAssignClassFor, setStaffToAssignClassFor] =
    React.useState<StaffResponse | null>(null)
  const [isAssignSubjectOpen, setIsAssignSubjectOpen] = React.useState(false)
  const [staffToAssignSubjectFor, setStaffToAssignSubjectFor] =
    React.useState<StaffResponse | null>(null)
  const [isWorkloadOpen, setIsWorkloadOpen] = React.useState(false)
  const [staffToViewWorkloadFor, setStaffToViewWorkloadFor] =
    React.useState<StaffResponse | null>(null)
  const [isAttendanceOpen, setIsAttendanceOpen] = React.useState(false)
  const [staffToManageAttendanceFor, setStaffToManageAttendanceFor] =
    React.useState<StaffResponse | null>(null)
  const [isLeavesOpen, setIsLeavesOpen] = React.useState(false)
  const [staffToManageLeavesFor, setStaffToManageLeavesFor] =
    React.useState<StaffResponse | null>(null)
  const [isPermissionSetsOpen, setIsPermissionSetsOpen] = React.useState(false)
  const [staffToManagePermissionSetsFor, setStaffToManagePermissionSetsFor] =
    React.useState<StaffResponse | null>(null)

  const staffQuery = useQuery({
    ...getAllStaffQueryOptions({
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
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
        sort_by: sortBy ?? 'created_at',
        sort_order:
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'desc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteStaff = useDeleteStaff()
  const createStaff = useCreateStaff()
  const updateStaff = useUpdateStaff()
  const bulkDeleteStaff = useBulkDeleteStaff()
  const bulkUpdateStaff = useBulkUpdateStaff()
  const assignClass = useAssignClassToTeacher()
  const assignSubject = useAssignSubjectToTeacher()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

  const fetchFullData = React.useCallback(async () => {
    const options = getAllStaffQueryOptions({
      query: {
        page: 1,
        limit: 1000,
        search: search ?? undefined,
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
        sort_by: sortBy ?? 'created_at',
        sort_order: sortOrder === 'desc' ? 'desc' : 'asc',
      },
    })

    if (!options.queryFn) return []
    const response = await options.queryFn({
      queryKey: options.queryKey,
      meta: undefined,
      client: queryClient,
      signal: new AbortController().signal,
    })
    return response.data || []
  }, [
    search,
    staffTypeFilter,
    employmentStatusFilter,
    sortBy,
    sortOrder,
    queryClient,
  ])

  const facetedFilters = React.useMemo<Array<DataTableFacetedFilter>>(
    () => [
      {
        columnId: 'staff_type',
        title: 'Role',
        options: [
          { label: 'Teaching', value: 'Teaching' },
          { label: 'Non-Teaching', value: 'NonTeaching' },
          { label: 'Admin', value: 'Admin' },
          { label: 'Support', value: 'Support' },
        ],
      },
      {
        columnId: 'employment_status',
        title: 'Status',
        options: [
          { label: 'Permanent', value: 'Permanent' },
          { label: 'Contract', value: 'Contract' },
          { label: 'Part-Time', value: 'PartTime' },
          { label: 'Probation', value: 'Probation' },
        ],
      },
    ],
    [],
  )

  const columns = getStaffColumns({
    onEdit: setStaffToEdit,
    onDelete: setStaffToDelete,
    onUploadPhoto: (staff) => {
      setStaffToUploadPhotoFor(staff)
      setIsUploadPhotoOpen(true)
    },
    onAssignClass: (staff) => {
      setStaffToAssignClassFor(staff)
      setIsAssignClassOpen(true)
    },
    onAssignSubject: (staff) => {
      setStaffToAssignSubjectFor(staff)
      setIsAssignSubjectOpen(true)
    },
    onViewWorkload: (staff) => {
      setStaffToViewWorkloadFor(staff)
      setIsWorkloadOpen(true)
    },
    onManageAttendance: (staff) => {
      setStaffToManageAttendanceFor(staff)
      setIsAttendanceOpen(true)
    },
    onManageLeaves: (staff) => {
      setStaffToManageLeavesFor(staff)
      setIsLeavesOpen(true)
    },
    onManagePermissions: (staff) => {
      setStaffToManagePermissionSetsFor(staff)
      setIsPermissionSetsOpen(true)
    },
  })

  return (
    <Stack gap={4} p={8} className="h-full">
      <StaffHeader totalStaff={staffQuery.data?.total} />
      <StaffFilters />
      <StaffListContainer
        staffQuery={staffQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
        setStaffToEdit={setStaffToEdit}
        setStaffToDelete={setStaffToDelete}
        setIsCreateStaffOpen={setIsCreateStaffOpen}
        onFetchFullData={fetchFullData}
        facetedFilters={facetedFilters}
        onAdd={() => setIsCreateStaffOpen(true)}
        onAddLabel="Add Staff"
        extraActions={
          <Tabs
            value={view ?? 'table'}
            onValueChange={(value: string) => setView(value)}
          >
            <TabsList className="h-8">
              <TabsTrigger value="table" className="gap-2 h-7 px-2">
                <HugeiconsIcon icon={TableIcon} className="size-3.5" />
                Table
              </TabsTrigger>
              <TabsTrigger value="grid" className="gap-2 h-7 px-2">
                <HugeiconsIcon icon={LayoutGridIcon} className="size-3.5" />
                Grid
              </TabsTrigger>
            </TabsList>
          </Tabs>
        }
        bulkActions={({ selectedRows }) => (
          <StaffBulkActionsToolbar
            selectedStaff={new Set(selectedRows.map((r) => r.id))}
            onBulkDelete={() => setIsBulkDeleteOpen(true)}
            onBulkEdit={() => setIsBulkEditOpen(true)}
          />
        )}
      />

      <StaffDeleteDialog
        staffToDeleteId={staffToDelete}
        setStaffToDeleteId={setStaffToDelete}
        onDeleteConfirm={(id) =>
          deleteStaff.mutate(
            { path: { staff_id: id } },
            {
              onSuccess: () => {
                setStaffToDelete(null)
              },
            },
          )
        }
      />

      <AlertDialog open={isBulkDeleteOpen} onOpenChange={setIsBulkDeleteOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete{' '}
              {Object.keys(rowSelection).filter((k) => rowSelection[k]).length}{' '}
              staff members and remove their data from our servers.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                bulkDeleteStaff.mutate(
                  {
                    body: {
                      staff_ids: Object.keys(rowSelection).filter(
                        (k) => rowSelection[k],
                      ),
                    },
                  },
                  {
                    onSuccess: () => {
                      setIsBulkDeleteOpen(false)
                      setRowSelection({})
                    },
                  },
                )
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
          bulkUpdateStaff.mutate(
            {
              body: {
                staff_ids: Object.keys(rowSelection).filter(
                  (k) => rowSelection[k],
                ),
                ...data,
              },
            },
            {
              onSuccess: () => {
                setIsBulkEditOpen(false)
                setRowSelection({})
              },
            },
          )
        }
        selectedCount={
          Object.keys(rowSelection).filter((k) => rowSelection[k]).length
        }
        isSubmitting={bulkUpdateStaff.isPending}
      />

      <StaffAddDialog
        isAddOpen={isCreateStaffOpen}
        setIsAddOpen={setIsCreateStaffOpen}
        onAddConfirm={(values) =>
          createStaff.mutate(
            { body: values },
            {
              onSuccess: () => {
                setIsCreateStaffOpen(false)
              },
            },
          )
        }
        isAdding={createStaff.isPending}
      />

      <StaffEditDialog
        staffToEdit={staffToEdit}
        setStaffToEdit={setStaffToEdit}
        onEditConfirm={(values) =>
          staffToEdit &&
          updateStaff.mutate(
            {
              path: { staff_id: staffToEdit.id },
              body: values,
            },
            {
              onSuccess: () => {
                setStaffToEdit(null)
              },
            },
          )
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
          assignClass.mutate(
            {
              path: { teacher_id: staffId },
              body: data,
            },
            {
              onSuccess: () => {
                setIsAssignClassOpen(false)
              },
            },
          )
        }
        isSubmitting={assignClass.isPending}
      />

      <StaffAssignSubjectDialog
        staff={staffToAssignSubjectFor}
        open={isAssignSubjectOpen}
        onOpenChange={setIsAssignSubjectOpen}
        onConfirm={(staffId, data) =>
          assignSubject.mutate(
            {
              path: { teacher_id: staffId },
              body: data,
            },
            {
              onSuccess: () => {
                setIsAssignSubjectOpen(false)
              },
            },
          )
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
    </Stack>
  )
}
