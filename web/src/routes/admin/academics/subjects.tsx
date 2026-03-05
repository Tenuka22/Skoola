import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'

import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  LayoutGridIcon,
  TableIcon,
} from '@hugeicons/core-free-icons'
import type { SubjectFormValues } from '@/features/academics/subjects/schemas'
import type { SubjectResponse } from '@/lib/api/types.gen'
import { SubjectsHeader } from '@/features/academics/subjects/components/subjects-header'
import { SubjectsListContainer } from '@/features/academics/subjects/components/subjects-list-container'
import { SubjectsGridView } from '@/features/academics/subjects/components/subjects-grid-view'
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
import { HStack, Stack } from '@/components/primitives'
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
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

export const Route = createFileRoute('/admin/academics/subjects')({
  component: SubjectsPage,
})

function SubjectsPage() {
  const queryClient = useQueryClient()
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
          sortOrder === 'asc' || sortOrder === 'desc' ? sortOrder : 'desc',
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

  const fetchFullData = React.useCallback(async () => {
    const options = getAllSubjectsQueryOptions({
      query: {
        page: 1,
        limit: 1000,
        search: search ?? undefined,
        sort_by: sortBy ?? 'subject_name_en',
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
    return response.data || []
  }, [search, sortBy, sortOrder, queryClient])

  const facetedFilters = React.useMemo(
    () => [
      {
        columnId: 'is_core',
        title: 'Type',
        options: [
          { label: 'Core', value: 'true' },
          { label: 'Elective', value: 'false' },
        ],
      },
    ],
    [],
  )

  const columns = getSubjectsColumns({
    onEdit: setSubjectToEdit,
    onDelete: setSubjectToDelete,
    onAssignToGrade: setSubjectToAssignToGrade,
    onAssignToStream: setSubjectToAssignToStream,
    onEnrollStudent: setSubjectToEnrollStudent,
    onViewEnrollments: setSubjectToViewEnrollments,
  })

  return (
    <Stack gap={4} p={8} className="h-full">
      <SubjectsHeader />

      <Tabs
        defaultValue="table"
        className="flex flex-col flex-1 gap-4 overflow-hidden"
      >
        <HStack>
          <TabsList>
            <TabsTrigger value="table" className="gap-2">
              <HugeiconsIcon icon={TableIcon} className="size-4" />
              Table
            </TabsTrigger>
            <TabsTrigger value="grid" className="gap-2">
              <HugeiconsIcon icon={LayoutGridIcon} className="size-4" />
              Grid
            </TabsTrigger>
          </TabsList>
        </HStack>

        <TabsContent value="table" className="flex-1 w-full mt-0">
          <div className="overflow-y-auto w-0 flex-1 min-w-full h-full">
            <SubjectsListContainer
              query={subjectsQuery}
              columns={columns}
              rowSelection={rowSelection}
              setRowSelection={setRowSelection}
              onFetchFullData={fetchFullData}
              facetedFilters={facetedFilters}
              onAdd={() => setIsCreateSubjectOpen(true)}
              onAddLabel="Add Subject"
              bulkActions={({ selectedRows }) => (
                <Button
                  variant="destructive"
                  size="sm"
                  onClick={() => setIsBulkDeleteOpen(true)}
                >
                  <HugeiconsIcon icon={Delete02Icon} className="size-4 mr-2" />
                  Delete Selected ({selectedRows.length})
                </Button>
              )}
            />
          </div>
        </TabsContent>

        <TabsContent
          value="grid"
          className="flex-1 w-full mt-0 overflow-y-auto"
        >
          <SubjectsGridView
            data={subjectsQuery.data?.data || []}
            isLoading={subjectsQuery.isLoading}
            onEdit={setSubjectToEdit}
            onDelete={setSubjectToDelete}
            onAssignToGrade={setSubjectToAssignToGrade}
            onAssignToStream={setSubjectToAssignToStream}
            onEnrollStudent={setSubjectToEnrollStudent}
            onViewEnrollments={setSubjectToViewEnrollments}
          />
        </TabsContent>
      </Tabs>

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
              {Object.keys(rowSelection).filter((k) => rowSelection[k]).length}{' '}
              subjects.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() => {
                const ids = Object.keys(rowSelection).filter(
                  (k) => rowSelection[k],
                )
                bulkDeleteSubjects.mutate(
                  {
                    body: { subject_ids: ids },
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
        onEnrollStudent={() => {
          setSubjectToEnrollStudent(subjectToViewEnrollments)
          setSubjectToViewEnrollments(null)
        }}
      />
    </Stack>
  )
}
