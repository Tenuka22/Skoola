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
import { StudentBulkDeleteDialog } from '../../features/students/components/student-bulk-delete-dialog'
import { StudentDeleteDialog } from '../../features/students/components/student-delete-dialog'
import { StudentEditDialog } from '../../features/students/components/student-edit-dialog'
import { StudentToolbar } from '../../features/students/components/student-toolbar'
import { getStudentColumns } from '../../features/students/components/student-table-columns'
import { StudentFilters } from '../../features/students/components/student-filters'
import { StudentHeader } from '../../features/students/components/student-header'
import { StudentListContainer } from '../../features/students/components/student-list-container'
import { useStudentsStore } from '../../features/students/store'
import { handleExportCSV } from '../../lib/export'
import { authClient } from '../../lib/clients'
import {
  deleteStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation,
  getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Options,
  getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1QueryKey,
  postStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Mutation,
  putStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation,
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
  } = store

  const sortBy = sorting[0]?.id
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  const studentsQuery = useQuery({
    ...getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Options({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        status: statusFilter === 'all' ? undefined : statusFilter,
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
      queryKey: getStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1QueryKey(),
    })
  }

  const deleteStudent = useMutation({
    ...deleteStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Student deleted successfully.`)
      invalidateStudents()
      store.setStudentToDelete(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to delete student: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const createStudent = useMutation({
    ...postStudents9Cfb76Aa83C6A83D99Db1D6755C24Ee1Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Student created successfully.`)
      invalidateStudents()
      store.setIsAddStudentOpen(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to create student: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateStudent = useMutation({
    ...putStudents4D5Cba944Bd069Fdf2A0246F5Bac2855Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success(`Student updated successfully.`)
      invalidateStudents()
      store.setStudentToEdit(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to update student: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const [rowSelection, setRowSelection] = React.useState({})
  const selectedStudents = React.useMemo(() => {
    return new Set(Object.keys(rowSelection))
  }, [rowSelection])

  const columns = getStudentColumns({
    onEdit: store.setStudentToEdit,
    onDelete: store.setStudentToDelete,
  })

  const students = studentsQuery.data?.data ?? []
  const totalStudents = studentsQuery.data?.total ?? 0

  return (
    <div className="flex h-full flex-col bg-background">
      <StudentHeader totalStudents={totalStudents} />
      <StudentToolbar
        onExport={() =>
          handleExportCSV(students, 'students_export.csv', [
            { header: 'Admission No', accessor: 'admission_number' },
            { header: 'Name', accessor: 'name_english' },
            { header: 'Email', accessor: 'email' },
            { header: 'Status', accessor: 'status' },
          ])
        }
        selectedStudents={selectedStudents}
        onBulkDelete={() => store.setIsBulkDeleteOpen(true)}
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
        isBottomToolbar
        selectedStudents={selectedStudents}
        onBulkDelete={() => store.setIsBulkDeleteOpen(true)}
        onExport={() => {}}
      />


      <StudentDeleteDialog
        studentToDelete={store.studentToDelete}
        setStudentToDelete={store.setStudentToDelete}
        onDeleteConfirm={(id) =>
          deleteStudent.mutate({ path: { student_id: id } })
        }
      />

      <StudentAddDialog
        isAddOpen={store.isAddStudentOpen}
        setIsAddOpen={store.setIsAddStudentOpen}
        onAddConfirm={(values) => createStudent.mutate({ body: values })}
        isAdding={createStudent.isPending}
      />

      <StudentEditDialog
        studentToEdit={store.studentToEdit}
        setStudentToEdit={store.setStudentToEdit}
        onEditConfirm={(values) =>
          store.studentToEdit &&
          updateStudent.mutate({
            path: { student_id: store.studentToEdit.id },
            body: values,
          })
        }
        isEditing={updateStudent.isPending}
      />

      <StudentBulkDeleteDialog
        isBulkDeleteOpen={store.isBulkDeleteOpen}
        setIsBulkDeleteOpen={store.setIsBulkDeleteOpen}
        onBulkDeleteConfirm={() => {
          toast.warning('Bulk delete is not implemented yet.')
          store.setIsBulkDeleteOpen(false)
        }}
        selectedCount={selectedStudents.size}
      />

    </div>
  )
}
