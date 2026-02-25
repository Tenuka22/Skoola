import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import { StudentAddDialog } from '../../features/students/components/student-add-dialog'
import { StudentModals } from '../../features/students/components/student-modals'
import { StudentPhotoUploadDialog } from '../../features/students/components/student-photo-upload-dialog'
import { StudentAssignClassDialog } from '../../features/students/components/student-assign-class-dialog'
import { StudentGuardiansDialog } from '../../features/students/components/student-guardians-dialog'
import { StudentAttendanceDialog } from '../../features/students/components/student-attendance-dialog'
import { StudentMarksDialog } from '../../features/students/components/student-marks-dialog'
import { StudentToolbar } from '../../features/students/components/student-toolbar'
import { getStudentColumns } from '../../features/students/components/student-table-columns'
import { StudentFilters } from '../../features/students/components/student-filters'
import { StudentHeader } from '../../features/students/components/student-header'
import { StudentListContainer } from '../../features/students/components/student-list-container'
import { StudentsToolbar } from '../../features/students/components/students-toolbar'
import { useStudentsStore } from '../../features/students/store'
import { handleExportCSV } from '../../lib/export'
import { authClient } from '../../lib/clients'
import type { UpdateStudentRequest } from '@/lib/api/types.gen'
import {
  assignStudentToClassMutation,
  createStudentMutation,
  deleteStudentMutation,
  getAllStudentsOptions,
  getAllStudentsQueryKey,
  updateStudentMutation,
} from '@/lib/api/@tanstack/react-query.gen'

export const Route = createFileRoute('/admin/students')({
  component: StudentsPage,
})

function StudentsPage() {
  const store = useStudentsStore()
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
    statusFilter,
    sorting,
    debouncedSearch,
    createdAfter,
    createdBefore,
    setIsBulkDeleteOpen,
    setIsBulkEditOpen,
    setIsCreateStudentOpen,
    isUploadPhotoOpen,
    setIsUploadPhotoOpen,
    studentToUploadPhotoFor,
    isAssignClassOpen,
    setIsAssignClassOpen,
    studentToAssignClassFor,
    isGuardiansOpen,
    setIsGuardiansOpen,
    studentToManageGuardiansFor,
    isAttendanceOpen,
    setIsAttendanceOpen,
    studentToManageAttendanceFor,
    isMarksOpen,
    setIsMarksOpen,
    studentToManageMarksFor,
  } = store

  const sortBy = sorting[0]?.id
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  const studentsQuery = useQuery({
    ...getAllStudentsOptions({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        status: statusFilter === 'all' ? undefined : (statusFilter as any),
        created_after: createdAfter ?? undefined,
        created_before: createdBefore ?? undefined,
        sort_by: sortBy,
        sort_order: sortOrder,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const queryClient = useQueryClient()
  const invalidateStudents = () => {
    queryClient.invalidateQueries({
      queryKey: getAllStudentsQueryKey(),
    })
  }

  const deleteStudent = useMutation({
    ...deleteStudentMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const identifier = variables?.path.student_id || 'Student'
      toast.success(`Successfully deleted ${identifier}.`)
      invalidateStudents()
      store.setStudentToDelete(null)
    },
    onError: (error, variables) => {
      const identifier = variables?.path.student_id || 'Student'
      toast.error(
        `Failed to delete ${identifier}: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const createStudent = useMutation({
    ...createStudentMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const identifier = variables?.body.name_english || 'New student'
      toast.success(`Student ${identifier} created successfully.`)
      invalidateStudents()
      store.setIsCreateStudentOpen(false)
    },
    onError: (error, variables) => {
      const identifier = variables?.body.name_english || 'Student'
      toast.error(
        `Failed to create ${identifier}: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateStudent = useMutation({
    ...updateStudentMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const identifier = variables?.path.student_id || 'Student'
      toast.success(`Successfully updated ${identifier}.`)
      invalidateStudents()
      store.setStudentToEdit(null)
    },
    onError: (error, variables) => {
      const identifier = variables?.path.student_id || 'Student'
      toast.error(
        `Failed to update ${identifier}: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const assignClass = useMutation({
    ...assignStudentToClassMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Class assigned successfully.')
      setIsAssignClassOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to assign class: ${error.message || 'Unknown error'}`)
    },
  })

  const [rowSelection, setRowSelection] = React.useState<Record<string, boolean>>({})
  const selectedStudents = React.useMemo(() => {
    return new Set(Object.keys(rowSelection).filter(k => rowSelection[k]))
  }, [rowSelection])

  const columns = getStudentColumns({
    onEdit: store.setStudentToEdit,
    onDelete: store.setStudentToDelete,
    onUploadPhoto: store.setStudentToUploadPhotoFor,
    onAssignClass: store.setStudentToAssignClassFor,
    onManageGuardians: store.setStudentToManageGuardiansFor,
    onManageAttendance: store.setStudentToManageAttendanceFor,
    onManageMarks: store.setStudentToManageMarksFor,
  })

  const totalStudents = studentsQuery.data?.total ?? 0

  return (
    <div className="flex h-full flex-col bg-background">
      <StudentHeader totalStudents={totalStudents} />
      <StudentsToolbar
        onExport={() =>
          handleExportCSV(
            studentsQuery.data?.data || [],
            'students_export.csv',
            [
              { header: 'Admission No', accessor: 'admission_number' },
              { header: 'Name', accessor: 'name_english' },
              { header: 'Email', accessor: 'email' },
              { header: 'Status', accessor: 'status' },
            ],
          )
        }
      />
      <StudentFilters />
      <StudentListContainer
        studentsQuery={studentsQuery}
        columns={columns}
        limit={limit}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <StudentToolbar
        selectedStudents={selectedStudents}
        onBulkDelete={() => setIsBulkDeleteOpen(true)}
        onBulkEdit={() => setIsBulkEditOpen(true)}
      />

      <StudentModals
        studentToDelete={store.studentToDelete}
        setStudentToDelete={store.setStudentToDelete}
        onDeleteConfirm={(id) =>
          deleteStudent.mutate({ path: { student_id: id } })
        }
        isBulkDeleteOpen={store.isBulkDeleteOpen}
        setIsBulkDeleteOpen={setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() => {
          toast.warning('Bulk delete is not implemented yet.')
          setIsBulkDeleteOpen(false)
        }}
        selectedCount={selectedStudents.size}
        studentToEdit={store.studentToEdit}
        setStudentToEdit={store.setStudentToEdit}
        onEditConfirm={(values: UpdateStudentRequest) =>
          store.studentToEdit &&
          updateStudent.mutate({
            path: { student_id: store.studentToEdit.id },
            body: values as any,
          })
        }
        isEditing={updateStudent.isPending}
      />

      <StudentAddDialog
        isAddOpen={store.isCreateStudentOpen}
        setIsAddOpen={setIsCreateStudentOpen}
        onAddConfirm={(values) => createStudent.mutate({ body: values as any })}
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
          assignClass.mutate({
            body: { ...data, student_id: studentId },
          })
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
    </div>
  )
}
