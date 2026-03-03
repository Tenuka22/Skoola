import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { LayoutGridIcon, TableIcon } from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'

import { StudentAddDialog } from '../../features/students/components/student-add-dialog'
import { StudentModals } from '../../features/students/components/student-modals'
import { StudentPhotoUploadDialog } from '../../features/students/components/student-photo-upload-dialog'
import { StudentAssignClassDialog } from '../../features/students/components/student-assign-class-dialog'
import { StudentGuardiansDialog } from '../../features/students/components/student-guardians-dialog'
import { StudentAttendanceDialog } from '../../features/students/components/student-attendance-dialog'
import { StudentMarksDialog } from '../../features/students/components/student-marks-dialog'
import { StudentBehaviorDialog } from '../../features/behavior-management/components/student-behavior-dialog'
import { StudentToolbar } from '../../features/students/components/student-toolbar'
import { StudentBulkAssignClassDialog } from '../../features/students/components/student-bulk-assign-class-dialog'
import { getStudentColumns } from '../../features/students/components/student-table-columns'
import { StudentFilters } from '../../features/students/components/student-filters'
import { StudentHeader } from '../../features/students/components/student-header'
import { StudentListContainer } from '../../features/students/components/student-list-container'
import { isStudentStatus } from '../../features/students/utils/student-guards'
import type { StudentResponse, UpdateStudentRequest } from '@/lib/api/types.gen'
import type { DataTableFacetedFilter } from '@/components/data-table'
import {
  getAllStudentsQueryOptions,
  useAssignStudentToClass,
  useCreateStudent,
  useDeleteStudent,
  useUpdateStudent,
} from '@/features/students/api'
import { useStudentsSearchParams } from '@/features/students/search-params'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Stack } from '@/components/primitives'

export const Route = createFileRoute('/admin/students')({
  component: StudentsPage,
})

function StudentsPage() {
  const queryClient = useQueryClient()
  const searchParams = useStudentsSearchParams()
  const {
    page,
    limit,
    search,
    statusFilter,
    createdAfter,
    createdBefore,
    sortBy,
    sortOrder,
  } = searchParams

  const [studentToDelete, setStudentToDelete] = React.useState<string | null>(
    null,
  )
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateStudentOpen, setIsCreateStudentOpen] = React.useState(false)
  const [studentToEdit, setStudentToEdit] =
    React.useState<StudentResponse | null>(null)
  const [isUploadPhotoOpen, setIsUploadPhotoOpen] = React.useState(false)
  const [studentToUploadPhotoFor, setStudentToUploadPhotoFor] =
    React.useState<StudentResponse | null>(null)
  const [isAssignClassOpen, setIsAssignClassOpen] = React.useState(false)
  const [studentToAssignClassFor, setStudentToAssignClassFor] =
    React.useState<StudentResponse | null>(null)
  const [isGuardiansOpen, setIsGuardiansOpen] = React.useState(false)
  const [studentToManageGuardiansFor, setStudentToManageGuardiansFor] =
    React.useState<StudentResponse | null>(null)
  const [isAttendanceOpen, setIsAttendanceOpen] = React.useState(false)
  const [studentToManageAttendanceFor, setStudentToManageAttendanceFor] =
    React.useState<StudentResponse | null>(null)
  const [isMarksOpen, setIsMarksOpen] = React.useState(false)
  const [studentToManageMarksFor, setStudentToManageMarksFor] =
    React.useState<StudentResponse | null>(null)
  const [isBehaviorOpen, setIsBehaviorOpen] = React.useState(false)
  const [studentToManageBehaviorFor, setStudentToManageBehaviorFor] =
    React.useState<StudentResponse | null>(null)
  const [isBulkAssignClassOpen, setIsBulkAssignClassOpen] =
    React.useState(false)

  const studentsQuery = useQuery({
    ...getAllStudentsQueryOptions({
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
        status:
          statusFilter === 'all'
            ? undefined
            : isStudentStatus(statusFilter)
              ? statusFilter
              : undefined,
        created_after: createdAfter ?? undefined,
        created_before: createdBefore ?? undefined,
        sort_by: sortBy ?? 'created_at',
        sort_order:
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'desc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const deleteStudent = useDeleteStudent()
  const createStudent = useCreateStudent()
  const updateStudent = useUpdateStudent()
  const assignClass = useAssignStudentToClass()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})

  const fetchFullData = React.useCallback(async () => {
    const options = getAllStudentsQueryOptions({
      query: {
        page: 1,
        limit: 1000,
        search: search ?? undefined,
        status:
          statusFilter === 'all'
            ? undefined
            : isStudentStatus(statusFilter)
              ? statusFilter
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
      signal: new AbortSignal(),
    })
    return response.data
  }, [search, statusFilter, sortBy, sortOrder, queryClient])

  const facetedFilters = React.useMemo<Array<DataTableFacetedFilter>>(
    () => [
      {
        columnId: 'status',
        title: 'Status',
        options: [
          { label: 'Active', value: 'Active' },
          { label: 'Inactive', value: 'Inactive' },
          { label: 'Graduated', value: 'Graduated' },
          { label: 'Withdrawn', value: 'Withdrawn' },
        ],
      },
      {
        columnId: 'gender',
        title: 'Gender',
        options: [
          { label: 'Male', value: 'Male' },
          { label: 'Female', value: 'Female' },
          { label: 'Other', value: 'Other' },
        ],
      },
    ],
    [],
  )

  const columns = getStudentColumns({
    onEdit: setStudentToEdit,
    onDelete: setStudentToDelete,
    onUploadPhoto: (student) => {
      setStudentToUploadPhotoFor(student)
      setIsUploadPhotoOpen(true)
    },
    onAssignClass: (student) => {
      setStudentToAssignClassFor(student)
      setIsAssignClassOpen(true)
    },
    onManageGuardians: (student) => {
      setStudentToManageGuardiansFor(student)
      setIsGuardiansOpen(true)
    },
    onManageAttendance: (student) => {
      setStudentToManageAttendanceFor(student)
      setIsAttendanceOpen(true)
    },
    onManageMarks: (student) => {
      setStudentToManageMarksFor(student)
      setIsMarksOpen(true)
    },
    onManageBehavior: (student) => {
      setStudentToManageBehaviorFor(student)
      setIsBehaviorOpen(true)
    },
  })

  const totalStudents = studentsQuery.data?.total ?? 0

  return (
    <Stack gap={4} p={8} className="h-full">
      <StudentHeader totalStudents={totalStudents} />
      <StudentFilters />
      <StudentListContainer
        studentsQuery={studentsQuery}
        columns={columns}
        limit={limit ?? 10}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
        setStudentToEdit={setStudentToEdit}
        setStudentToDelete={setStudentToDelete}
        onFetchFullData={fetchFullData}
        facetedFilters={facetedFilters}
        onAdd={() => setIsCreateStudentOpen(true)}
        onAddLabel="Add Student"
        extraActions={
          <Tabs
            value={searchParams.view ?? 'table'}
            onValueChange={(value: string) => searchParams.setView(value)}
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
          <StudentToolbar
            selectedStudents={new Set(selectedRows.map((r) => r.id))}
            onBulkDelete={() => setIsBulkDeleteOpen(true)}
            onBulkEdit={() => {}}
            onBulkAssignClass={() => setIsBulkAssignClassOpen(true)}
          />
        )}
      />

      <StudentModals
        studentToDelete={studentToDelete}
        setStudentToDelete={setStudentToDelete}
        onDeleteConfirm={(id) =>
          deleteStudent.mutate(
            { path: { student_id: id } },
            {
              onSuccess: () => {
                setStudentToDelete(null)
              },
            },
          )
        }
        isBulkDeleteOpen={isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() => {
          setIsBulkDeleteOpen(false)
        }}
        selectedCount={
          Object.keys(rowSelection).filter((k) => rowSelection[k]).length
        }
        studentToEdit={studentToEdit}
        setStudentToEdit={setStudentToEdit}
        onEditConfirm={(values: UpdateStudentRequest) =>
          studentToEdit &&
          updateStudent.mutate(
            {
              path: { student_id: studentToEdit.id },
              body: values,
            },
            {
              onSuccess: () => {
                setStudentToEdit(null)
              },
            },
          )
        }
        isEditing={updateStudent.isPending}
      />

      <StudentAddDialog
        isAddOpen={isCreateStudentOpen}
        setIsAddOpen={setIsCreateStudentOpen}
        onAddConfirm={(values) =>
          createStudent.mutate(
            { body: values },
            {
              onSuccess: () => {
                setIsCreateStudentOpen(false)
              },
            },
          )
        }
        isAdding={createStudent.isPending}
      />

      <StudentPhotoUploadDialog
        student={studentToUploadPhotoFor}
        open={isUploadPhotoOpen}
        onOpenChange={setIsUploadPhotoOpen}
      />

      <StudentAssignClassDialog
        student={studentToAssignClassFor}
        open={isAssignClassOpen}
        onOpenChange={setIsAssignClassOpen}
        onConfirm={(studentId, data) =>
          assignClass.mutate(
            {
              body: { ...data, student_id: studentId },
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

      <StudentGuardiansDialog
        student={studentToManageGuardiansFor}
        open={isGuardiansOpen}
        onOpenChange={setIsGuardiansOpen}
      />

      <StudentAttendanceDialog
        student={studentToManageAttendanceFor}
        open={isAttendanceOpen}
        onOpenChange={setIsAttendanceOpen}
      />

      <StudentMarksDialog
        student={studentToManageMarksFor}
        open={isMarksOpen}
        onOpenChange={setIsMarksOpen}
      />

      <StudentBehaviorDialog
        student={studentToManageBehaviorFor}
        open={isBehaviorOpen}
        onOpenChange={setIsBehaviorOpen}
      />

      <StudentBulkAssignClassDialog
        selectedStudentIds={
          new Set(Object.keys(rowSelection).filter((k) => rowSelection[k]))
        }
        open={isBulkAssignClassOpen}
        onOpenChange={setIsBulkAssignClassOpen}
      />
    </Stack>
  )
}
