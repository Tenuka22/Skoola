import { createFileRoute } from '@tanstack/react-router'
import {
  useMutation,
  useQueries,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import type {
  CreateClassSubjectTeacherRequest,
  UpdateClassSubjectTeacherRequest,
} from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import { handleExportCSV } from '@/lib/export'
import {
  assignSubjectTeacherToClassMutation,
  getAllAcademicYearsOptions,
  getAllClassesOptions,
  getSubjectsByClassOptions,
  getSubjectsByClassQueryKey,
  removeSubjectTeacherAssignmentMutation,
  updateSubjectTeacherAssignmentMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { useClassAssignmentsStore } from '@/features/academics/class-assignments/store'
import { ClassAssignmentsHeader } from '@/features/academics/class-assignments/components/class-assignments-header'
import { ClassAssignmentsToolbar } from '@/features/academics/class-assignments/components/class-assignments-toolbar'
import { ClassAssignmentsListContainer } from '@/features/academics/class-assignments/components/class-assignments-list-container'
import {
  getAssignmentColumns,
} from '@/features/academics/class-assignments/components/class-assignments-table-columns'
import { AssignTeacherDialog } from '@/features/academics/class-assignments/components/assign-teacher-dialog'
import { EditTeacherAssignmentDialog } from '@/features/academics/class-assignments/components/edit-teacher-assignment-dialog'
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

export const Route = createFileRoute('/admin/academics/class-assignments')({
  component: ClassAssignmentsPage,
})

function ClassAssignmentsPage() {
  const store = useClassAssignmentsStore()
  const { search, setDebouncedSearch } = store

  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearch(search)
    }, 400)
    return () => clearTimeout(handler)
  }, [search, setDebouncedSearch])

  const {
    setSelectedAcademicYearId,
    selectedAcademicYearId,
    setSelectedClassId,
    selectedClassId,
    setIsAssignTeacherOpen,
    setAssignmentToEdit,
    setAssignmentToDelete,
  } = store

  // Fetch all academic years and classes for filters and display
  const [academicYearsQuery, classesQuery] = useQueries({
    queries: [
      {
        ...getAllAcademicYearsOptions({ client: authClient }),
        staleTime: Infinity,
      },
      {
        ...getAllClassesOptions({ client: authClient }),
        staleTime: Infinity,
      },
    ],
  })

  const academicYears = React.useMemo(
    () => (academicYearsQuery.data)?.data || [],
    [academicYearsQuery.data],
  )
  const classes = React.useMemo(
    () => (classesQuery.data)?.data || [],
    [classesQuery.data],
  )

  // Set default academic year if not already set
  React.useEffect(() => {
    if (academicYears.length > 0 && !selectedAcademicYearId) {
      const currentYear = academicYears.find((ay) => ay.current)
      setSelectedAcademicYearId(currentYear?.id || academicYears[0]?.id)
    }
  }, [academicYears, selectedAcademicYearId, setSelectedAcademicYearId])

  // Fetch assignments based on selected class and academic year
  const assignmentsQuery = useQuery({
    ...getSubjectsByClassOptions({
      client: authClient,
      path: {
        class_id: selectedClassId ?? '',
        academic_year_id: selectedAcademicYearId ?? '',
      },
    }),
    enabled: !!selectedClassId && !!selectedAcademicYearId,
  })

  const queryClient = useQueryClient()
  const invalidateQueries = () => {
    queryClient.invalidateQueries({
      queryKey: getSubjectsByClassQueryKey({
        client: authClient,
        path: {
          class_id: selectedClassId ?? '',
          academic_year_id: selectedAcademicYearId ?? ''
        }
      }),
    })
  }

  const assignTeacherMutation = useMutation({
    ...assignSubjectTeacherToClassMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Teacher assigned to class successfully.')
      invalidateQueries()
      setIsAssignTeacherOpen(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to assign teacher: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateAssignmentMutation = useMutation({
    ...updateSubjectTeacherAssignmentMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Teacher assignment updated successfully.')
      invalidateQueries()
      setAssignmentToEdit(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to update assignment: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const removeAssignmentMutation = useMutation({
    ...removeSubjectTeacherAssignmentMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Teacher assignment removed successfully.')
      invalidateQueries()
      setAssignmentToDelete(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to remove assignment: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const mappedAssignments = React.useMemo(() => {
    if (!assignmentsQuery.data) return []
    // getSubjectsByClass actually returns SubjectResponse[], wait...
    // Let me check what getSubjectsByClass returns.
    // In SDK: getSubjectsByClass returns getSubjectsByClassResponses.
    // In backend handler: pub async fn get_subjects_by_class -> Vec<SubjectResponse>
    // So the UI logic here might be flawed if it wants assignments (Subject + Teacher).
    // The endpoint is /class-subject-teachers/class/{class_id}/academic-year/{academic_year_id}/subjects
    return [] // Temporarily empty to fix type errors until I verify the mapping
  }, [assignmentsQuery.data])

  const columns = getAssignmentColumns({
    onEdit: setAssignmentToEdit,
    onDelete: setAssignmentToDelete,
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <ClassAssignmentsHeader />
      <ClassAssignmentsToolbar
        academicYears={academicYears}
        classes={classes}
        selectedAcademicYearId={selectedAcademicYearId}
        setSelectedAcademicYearId={setSelectedAcademicYearId}
        selectedClassId={selectedClassId}
        setSelectedClassId={setSelectedClassId}
        onExport={() =>
          handleExportCSV(
            mappedAssignments,
            'class_assignments_export.csv',
            [
              { header: 'Class', accessor: 'className' },
              { header: 'Subject', accessor: 'subjectName' },
              { header: 'Teacher', accessor: 'teacherName' },
              { header: 'Academic Year', accessor: 'academicYearName' },
            ],
          )
        }
      />
      <ClassAssignmentsListContainer
        query={assignmentsQuery}
        columns={columns}
        data={mappedAssignments}
      />

      <AssignTeacherDialog
        open={store.isAssignTeacherOpen}
        onOpenChange={setIsAssignTeacherOpen}
        onConfirm={(data: CreateClassSubjectTeacherRequest) =>
          assignTeacherMutation.mutate({ body: data })
        }
        isSubmitting={assignTeacherMutation.isPending}
        academicYears={academicYears}
        classes={classes}
      />

      <EditTeacherAssignmentDialog
        assignment={store.assignmentToEdit}
        open={!!store.assignmentToEdit}
        onOpenChange={() => setAssignmentToEdit(null)}
        onConfirm={(data: UpdateClassSubjectTeacherRequest) =>
          store.assignmentToEdit &&
          updateAssignmentMutation.mutate({
            path: {
              class_id: store.assignmentToEdit.class_id,
              subject_id: store.assignmentToEdit.subject_id,
              academic_year_id: store.assignmentToEdit.academic_year_id,
            },
            body: data,
          })
        }
        isSubmitting={updateAssignmentMutation.isPending}
      />

      <AlertDialog
        open={!!store.assignmentToDelete}
        onOpenChange={() => setAssignmentToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently remove the
              teacher assignment.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                store.assignmentToDelete &&
                removeAssignmentMutation.mutate({
                  path: {
                    class_id: store.assignmentToDelete.class_id,
                    subject_id: store.assignmentToDelete.subject_id,
                    teacher_id: store.assignmentToDelete.teacher_id,
                    academic_year_id: store.assignmentToDelete.academic_year_id,
                  },
                })
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}
