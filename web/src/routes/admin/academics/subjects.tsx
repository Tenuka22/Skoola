import { createFileRoute } from '@tanstack/react-router'
import { keepPreviousData, useQuery } from '@tanstack/react-query'
import * as React from 'react'

import type { SubjectFormValues } from '@/features/academics/subjects/schemas'
import type { SubjectResponse } from '@/lib/api/types.gen'
import { handleExportCSV } from '@/lib/export'
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
import { Stack } from '@/components/primitives'
import {
  getAllSubjectsQueryOptions,
  useAssignSubjectToGrade,
  useAssignSubjectToStream,
  useBulkDeleteSubjects,
  useCreateSubject,
  useDeleteSubject,
  useEnrollStudentInSubject,
  useUpdateSubject,
} from '@/features/academics/subjects/api'
import { useSubjectsSearchParams } from '@/features/academics/subjects/search-params'

export const Route = createFileRoute('/admin/academics/subjects')({
  component: SubjectsPage,
})

function SubjectsPage() {
  const { page, limit, search, sortBy, sortOrder } = useSubjectsSearchParams()

  const [subjectToDelete, setSubjectToDelete] = React.useState<string | null>(
    null,
  )
  const [isBulkDeleteOpen, setIsBulkDeleteOpen] = React.useState(false)
  const [isCreateSubjectOpen, setIsCreateSubjectOpen] = React.useState(false)
  const [subjectToEdit, setSubjectToEdit] =
    React.useState<SubjectResponse | null>(null)
  const [subjectToAssignToGrade, setSubjectToAssignToGrade] =
    React.useState<SubjectResponse | null>(null)
  const [subjectToAssignToStream, setSubjectToAssignToStream] =
    React.useState<SubjectResponse | null>(null)
  const [subjectToEnrollStudent, setSubjectToEnrollStudent] =
    React.useState<SubjectResponse | null>(null)
  const [subjectToViewEnrollments, setSubjectToViewEnrollments] =
    React.useState<SubjectResponse | null>(null)

  const subjectsQuery = useQuery({
    ...getAllSubjectsQueryOptions({
      query: {
        page: page ?? 1,
        limit: limit ?? 10,
        search: search ?? undefined,
        sort_by: sortBy ?? 'subject_name_en',
        sort_order:
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'asc',
      },
    }),
    placeholderData: keepPreviousData,
  })

  const createSubject = useCreateSubject()

  const updateSubject = useUpdateSubject()

  const deleteSubject = useDeleteSubject()

  const bulkDeleteSubjects = useBulkDeleteSubjects()

  const assignSubjectToGrade = useAssignSubjectToGrade()

  const assignSubjectToStream = useAssignSubjectToStream()

  const enrollStudentInSubject = useEnrollStudentInSubject()

  const [rowSelection, setRowSelection] = React.useState<
    Record<string, boolean>
  >({})
  const selectedSubjects = React.useMemo(() => {
    return new Set(Object.keys(rowSelection).filter((key) => rowSelection[key]))
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
    <Stack gap={0} className="h-full bg-background">
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
              {
                header: 'Is Core',
                accessor: (s: SubjectResponse) => (s.is_core ? 'Yes' : 'No'),
              },
            ],
          )
        }
        setIsCreateSubjectOpen={setIsCreateSubjectOpen}
      />
      <SubjectsListContainer
        query={subjectsQuery}
        columns={columns}
        rowSelection={rowSelection}
        setRowSelection={setRowSelection}
      />

      <SubjectAddDialog
        open={isCreateSubjectOpen}
        onOpenChange={setIsCreateSubjectOpen}
        onConfirm={(data: SubjectFormValues) =>
          createSubject.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsCreateSubjectOpen(false)
              },
            },
          )
        }
        isSubmitting={createSubject.isPending}
      />

      <SubjectEditDialog
        subject={subjectToEdit}
        open={!!subjectToEdit}
        onOpenChange={() => setSubjectToEdit(null)}
        onConfirm={(data: SubjectFormValues) =>
          subjectToEdit &&
          updateSubject.mutate(
            {
              path: { id: subjectToEdit.id },
              body: data,
            },
            {
              onSuccess: () => {
                setSubjectToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateSubject.isPending}
      />

      <AlertDialog
        open={!!subjectToDelete}
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
                subjectToDelete &&
                deleteSubject.mutate(
                  { path: { id: subjectToDelete } },
                  {
                    onSuccess: () => {
                      setSubjectToDelete(null)
                    },
                  },
                )
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog open={isBulkDeleteOpen} onOpenChange={setIsBulkDeleteOpen}>
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
                bulkDeleteSubjects.mutate(
                  {
                    body: { subject_ids: Array.from(selectedSubjects) },
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

      <SubjectAssignToGradeDialog
        subject={subjectToAssignToGrade}
        open={!!subjectToAssignToGrade}
        onOpenChange={() => setSubjectToAssignToGrade(null)}
        onConfirm={(gradeId) =>
          subjectToAssignToGrade &&
          assignSubjectToGrade.mutate(
            {
              body: {
                subject_id: subjectToAssignToGrade.id,
                grade_id: gradeId,
              },
            },
            {
              onSuccess: () => {
                setSubjectToAssignToGrade(null)
              },
            },
          )
        }
        isSubmitting={assignSubjectToGrade.isPending}
      />

      <SubjectAssignToStreamDialog
        subject={subjectToAssignToStream}
        open={!!subjectToAssignToStream}
        onOpenChange={() => setSubjectToAssignToStream(null)}
        onConfirm={(streamId) =>
          subjectToAssignToStream &&
          assignSubjectToStream.mutate(
            {
              body: {
                subject_id: subjectToAssignToStream.id,
                stream_id: streamId,
              },
            },
            {
              onSuccess: () => {
                setSubjectToAssignToStream(null)
              },
            },
          )
        }
        isSubmitting={assignSubjectToStream.isPending}
      />

      <SubjectEnrollStudentDialog
        subject={subjectToEnrollStudent}
        open={!!subjectToEnrollStudent}
        onOpenChange={() => setSubjectToEnrollStudent(null)}
        onConfirm={(studentId, academicYearId) =>
          subjectToEnrollStudent &&
          enrollStudentInSubject.mutate(
            {
              body: {
                subject_id: subjectToEnrollStudent.id,
                student_id: studentId,
                academic_year_id: academicYearId,
              },
            },
            {
              onSuccess: () => {
                setSubjectToEnrollStudent(null)
              },
            },
          )
        }
        isSubmitting={enrollStudentInSubject.isPending}
      />

      <SubjectEnrollmentsDialog
        subject={subjectToViewEnrollments}
        open={!!subjectToViewEnrollments}
        onOpenChange={() => setSubjectToViewEnrollments(null)}
      />
    </Stack>
  )
}
