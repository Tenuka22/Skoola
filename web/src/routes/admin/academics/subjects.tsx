import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import type { SubjectFormValues } from '@/features/academics/subjects/schemas'
import type { SubjectResponse } from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import { handleExportCSV } from '@/lib/export'
import {
  assignSubjectToGradeMutation,
  assignSubjectToStreamMutation,
  bulkDeleteSubjectsMutation,
  createSubjectMutation,
  deleteSubjectMutation,
  enrollStudentInSubjectMutation,
  getAllSubjectsOptions,
  getAllSubjectsQueryKey,
  updateSubjectMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { useSubjectsStore } from '@/features/academics/subjects/store'
import { SubjectsHeader } from '@/features/academics/subjects/components/subjects-header'
import { SubjectsToolbar } from '@/features/academics/subjects/components/subjects-toolbar'
import { SubjectsListContainer } from '@/features/academics/subjects/components/subjects-list-container'
import { getSubjectsColumns } from '@/features/academics/subjects/components/subjects-table-columns'
import { SubjectAddDialog } from '@/features/academics/subjects/components/subject-add-dialog'
import { SubjectEditDialog } from '@/features/academics/subjects/components/subject-edit-dialog'
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
import { SubjectAssignToGradeDialog } from '@/features/academics/subjects/components/subject-assign-to-grade-dialog'
import { SubjectAssignToStreamDialog } from '@/features/academics/subjects/components/subject-assign-to-stream-dialog'
import { SubjectEnrollStudentDialog } from '@/features/academics/subjects/components/subject-enroll-student-dialog'
import { SubjectEnrollmentsDialog } from '@/features/academics/subjects/components/subject-enrollments-dialog'

export const Route = createFileRoute('/admin/academics/subjects')({
  component: SubjectsPage,
})

function SubjectsPage() {
  const store = useSubjectsStore()
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
    sorting,
    debouncedSearch,
    setSubjectToEdit,
    setSubjectToDelete,
    setIsCreateSubjectOpen,
    isBulkDeleteOpen,
    setIsBulkDeleteOpen,
    subjectToAssignToGrade,
    setSubjectToAssignToGrade,
    subjectToAssignToStream,
    setSubjectToAssignToStream,
    subjectToEnrollStudent,
    setSubjectToEnrollStudent,
    subjectToViewEnrollments,
    setSubjectToViewEnrollments,
  } = store

  const sortBy = sorting[0]?.id
  const sortOrder = sorting[0]?.desc ? 'desc' : 'asc'

  const subjectsQuery = useQuery({
    ...getAllSubjectsOptions({
      client: authClient,
      query: {
        page,
        limit,
        search: debouncedSearch,
        sort_by: sortBy,
        sort_order: sortOrder,
      },
    }),
    placeholderData: keepPreviousData,
  })

  const queryClient = useQueryClient()
  const invalidateQueries = () => {
    queryClient.invalidateQueries({
      queryKey: getAllSubjectsQueryKey(),
    })
  }

  const createSubject = useMutation({
    ...createSubjectMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Subject created successfully.')
      invalidateQueries()
      setIsCreateSubjectOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to create subject: ${error.message || 'Unknown error'}`)
    },
  })

  const updateSubject = useMutation({
    ...updateSubjectMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Subject updated successfully.')
      invalidateQueries()
      setSubjectToEdit(null)
    },
    onError: (error) => {
      toast.error(`Failed to update subject: ${error.message || 'Unknown error'}`)
    },
  })

  const deleteSubject = useMutation({
    ...deleteSubjectMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Subject deleted successfully.')
      invalidateQueries()
      setSubjectToDelete(null)
    },
    onError: (error) => {
      toast.error(`Failed to delete subject: ${error.message || 'Unknown error'}`)
    },
  })

  const bulkDeleteSubjects = useMutation({
    ...bulkDeleteSubjectsMutation({
      client: authClient,
    }),
    onSuccess: (_, variables) => {
      const count = variables.body?.subject_ids?.length ?? 0
      toast.success(`Successfully deleted ${count} subjects.`)
      invalidateQueries()
      setIsBulkDeleteOpen(false)
      setRowSelection({})
    },
    onError: (error) => {
      toast.error(`Failed to delete subjects: ${error.message || 'Unknown error'}`)
    },
  })

  const assignSubjectToGrade = useMutation({
    ...assignSubjectToGradeMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Subject assigned to grade successfully.')
      invalidateQueries()
      setSubjectToAssignToGrade(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to assign subject to grade: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const assignSubjectToStream = useMutation({
    ...assignSubjectToStreamMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Subject assigned to stream successfully.')
      invalidateQueries()
      setSubjectToAssignToStream(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to assign subject to stream: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const enrollStudentInSubject = useMutation({
    ...enrollStudentInSubjectMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Student enrolled in subject successfully.')
      invalidateQueries()
      setSubjectToEnrollStudent(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to enroll student in subject: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const [rowSelection, setRowSelection] = React.useState<Record<string, boolean>>({})
  const selectedSubjects = React.useMemo(() => {
    return new Set(
      Object.keys(rowSelection).filter((key) => rowSelection[key]),
    )
  }, [rowSelection])

  const columns = getSubjectsColumns({
    onEdit: setSubjectToEdit,
    onDelete: setSubjectToDelete,
    onAssignToGrade: setSubjectToAssignToGrade,
    onAssignToStream: setSubjectToAssignToStream,
    onEnrollStudent: setSubjectToEnrollStudent,
    onViewEnrollments: setSubjectToViewEnrollments,
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <SubjectsHeader />
      <SubjectsToolbar
        onExport={() =>
          handleExportCSV(
            subjectsQuery.data?.data || [],
            'subjects_export.csv',
            [
              { header: 'ID', accessor: 'id' },
              { header: 'Name', accessor: 'subject_name_en' },
              { header: 'Code', accessor: 'subject_code' },
              { header: 'Is Core', accessor: (s: SubjectResponse) => s.is_core ? 'Yes' : 'No' },
            ],
          )
        }
      />
      <SubjectsListContainer
        query={subjectsQuery as any}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <SubjectAddDialog
        open={store.isCreateSubjectOpen}
        onOpenChange={setIsCreateSubjectOpen}
        onConfirm={(data: SubjectFormValues) =>
          createSubject.mutate({ body: data as any })
        }
        isSubmitting={createSubject.isPending}
      />

      <SubjectEditDialog
        subject={store.subjectToEdit}
        open={!!store.subjectToEdit}
        onOpenChange={() => setSubjectToEdit(null)}
        onConfirm={(data: SubjectFormValues) =>
          store.subjectToEdit &&
          updateSubject.mutate({
            path: { id: store.subjectToEdit.id },
            body: data as any,
          })
        }
        isSubmitting={updateSubject.isPending}
      />

      <AlertDialog
        open={!!store.subjectToDelete}
        onOpenChange={() => setSubjectToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              subject.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                store.subjectToDelete &&
                deleteSubject.mutate({ path: { id: store.subjectToDelete } })
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog
        open={isBulkDeleteOpen}
        onOpenChange={setIsBulkDeleteOpen}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete{' '}
              {selectedSubjects.size} subjects.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                bulkDeleteSubjects.mutate({
                  body: { subject_ids: Array.from(selectedSubjects) },
                })
              }}
            >
              Delete All
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <SubjectAssignToGradeDialog
        subject={subjectToAssignToGrade}
        open={!!subjectToAssignToGrade}
        onOpenChange={() => setSubjectToAssignToGrade(null)}
        onConfirm={(gradeId) =>
          subjectToAssignToGrade &&
          assignSubjectToGrade.mutate({
            body: {
              subject_id: subjectToAssignToGrade.id,
              grade_id: gradeId,
            },
          })
        }
        isSubmitting={assignSubjectToGrade.isPending}
      />

      <SubjectAssignToStreamDialog
        subject={subjectToAssignToStream}
        open={!!subjectToAssignToStream}
        onOpenChange={() => setSubjectToAssignToStream(null)}
        onConfirm={(streamId) =>
          subjectToAssignToStream &&
          assignSubjectToStream.mutate({
            body: {
              subject_id: subjectToAssignToStream.id,
              stream_id: streamId,
            },
          })
        }
        isSubmitting={assignSubjectToStream.isPending}
      />

      <SubjectEnrollStudentDialog
        subject={subjectToEnrollStudent}
        open={!!subjectToEnrollStudent}
        onOpenChange={() => setSubjectToEnrollStudent(null)}
        onConfirm={(studentId, academicYearId) =>
          subjectToEnrollStudent &&
          enrollStudentInSubject.mutate({
            body: {
              subject_id: subjectToEnrollStudent.id,
              student_id: studentId,
              academic_year_id: academicYearId,
            },
          })
        }
        isSubmitting={enrollStudentInSubject.isPending}
      />

      <SubjectEnrollmentsDialog
        subject={subjectToViewEnrollments}
        open={!!subjectToViewEnrollments}
        onOpenChange={() => setSubjectToViewEnrollments(null)}
      />
    </div>
  )
}
