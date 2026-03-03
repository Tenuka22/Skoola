import { createFileRoute } from '@tanstack/react-router'
import { useQueries, useQuery } from '@tanstack/react-query'
import * as React from 'react'

import type {
  ClassSubjectTeacherResponse,
  CreateClassSubjectTeacherRequest,
  SubjectResponse,
  UpdateClassSubjectTeacherRequest,
} from '@/lib/api/types.gen'
import type {
  ClassAssignmentRow, // Added
} from '@/features/academics/class-assignments/components/class-assignments-table-columns'
import { authClient } from '@/lib/clients'
import { handleExportCSV } from '@/lib/export'
import {
  getAllAcademicYearsOptions,
  getAllClassesOptions,
  getAllStaffOptions,
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { ClassAssignmentsHeader } from '@/features/academics/class-assignments/components/class-assignments-header'
import { ClassAssignmentsToolbar } from '@/features/academics/class-assignments/components/class-assignments-toolbar'
import { ClassAssignmentsListContainer } from '@/features/academics/class-assignments/components/class-assignments-list-container'
import {
  getAssignmentColumns,
  mapAssignmentsForTable, // Added
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
import { useClassAssignmentsSearchParams } from '@/features/academics/class-assignments/search-params'
import {
  getSubjectsByClassQueryOptions,
  useAssignSubjectTeacherToClass,
  useRemoveSubjectTeacherAssignment,
  useUpdateSubjectTeacherAssignment,
} from '@/features/academics/class-assignments/api'

export const Route = createFileRoute('/admin/academics/class-assignments')({
  component: ClassAssignmentsPage,
})

function ClassAssignmentsPage() {
  const {
    selectedAcademicYearId,
    setSelectedAcademicYearId,
    selectedClassId,
    setSelectedClassId,
  } = useClassAssignmentsSearchParams()

  const [isAssignTeacherOpen, setIsAssignTeacherOpen] = React.useState(false)
  const [assignmentToEdit, setAssignmentToEdit] =
    React.useState<ClassAssignmentRow | null>(null)
  const [assignmentToDelete, setAssignmentToDelete] =
    React.useState<ClassAssignmentRow | null>(null)

  // Fetch all academic years and classes for filters and display
  const [academicYearsQuery, classesQuery, subjectsQuery, staffQuery] = // Modified
    useQueries({
      queries: [
        {
          ...getAllAcademicYearsOptions({ client: authClient }),
          staleTime: Infinity,
        },
        {
          ...getAllClassesOptions({ client: authClient }),
          staleTime: Infinity,
        },
        {
          ...getAllSubjectsOptions({ client: authClient }), // Added
          staleTime: Infinity,
        },
        {
          ...getAllStaffOptions({ client: authClient }), // Added
          staleTime: Infinity,
        },
      ],
    })

  const academicYears = React.useMemo(
    () => academicYearsQuery.data?.data || [],
    [academicYearsQuery.data],
  )
  const classes = React.useMemo(
    () => classesQuery.data?.data || [],
    [classesQuery.data],
  )
  const subjects = React.useMemo(
    () => subjectsQuery.data?.data || [],
    [subjectsQuery.data],
  )
  const staff = React.useMemo(
    () => staffQuery.data?.data || [],
    [staffQuery.data],
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
    ...getSubjectsByClassQueryOptions({
      path: {
        class_id: selectedClassId ?? '',
        academic_year_id: selectedAcademicYearId ?? '',
      },
    }),
    enabled: !!selectedClassId && !!selectedAcademicYearId,
  })

  const assignTeacherMutation = useAssignSubjectTeacherToClass()

  const updateAssignmentMutation = useUpdateSubjectTeacherAssignment()

  const removeAssignmentMutation = useRemoveSubjectTeacherAssignment()

  const mappedAssignments = React.useMemo(() => {
    if (assignmentsQuery.isSuccess && assignmentsQuery.data) {
      // WORKAROUND: The API endpoint /class-subject-teachers/class/{class_id}/academic-year/{academic_year_id}/subjects
      // currently returns SubjectResponse[] instead of ClassSubjectTeacherResponse[].
      // This creates dummy ClassSubjectTeacherResponse objects for type compatibility with mapAssignmentsForTable.
      // The backend API needs to be updated to return actual ClassSubjectTeacherResponse[] for this endpoint.
      const dummyClassSubjectTeacherResponses: Array<ClassSubjectTeacherResponse> =
        assignmentsQuery.data.map((subject: SubjectResponse) => ({
          academic_year_id: selectedAcademicYearId ?? '',
          class_id: selectedClassId ?? '',
          subject_id: subject.id,
          teacher_id: '', // Placeholder, as actual teacher assignment is missing from SubjectResponse
          created_at: '',
          updated_at: '',
        }))
      return mapAssignmentsForTable(
        dummyClassSubjectTeacherResponses,
        academicYears,
        classes,
        subjects,
        staff,
      )
    }
    return []
  }, [
    assignmentsQuery.isSuccess,
    assignmentsQuery.data,
    academicYears,
    classes,
    subjects,
    staff,
    selectedAcademicYearId,
    selectedClassId,
  ])

  const columns = getAssignmentColumns({
    onEdit: (assignment) => setAssignmentToEdit(assignment), // Removed as any
    onDelete: (assignment) => setAssignmentToDelete(assignment), // Removed as any
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <ClassAssignmentsHeader />
      <ClassAssignmentsToolbar
        academicYears={academicYears}
        classes={classes}
        selectedAcademicYearId={selectedAcademicYearId ?? undefined}
        setSelectedAcademicYearId={(val) =>
          setSelectedAcademicYearId(val ?? null)
        }
        selectedClassId={selectedClassId ?? undefined}
        setSelectedClassId={(val) => setSelectedClassId(val ?? null)}
        setIsAssignTeacherOpen={setIsAssignTeacherOpen}
        onExport={() =>
          handleExportCSV(
            mappedAssignments.map((assignment) => ({
              className: assignment.className,
              subjectName: assignment.subjectName,
              teacherName: assignment.teacherName,
              academicYearName: assignment.academicYearName,
            })),
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
        query={assignmentsQuery} // Removed as any
        columns={columns}
        data={mappedAssignments} // Removed as any
      />

      <AssignTeacherDialog
        open={isAssignTeacherOpen}
        onOpenChange={setIsAssignTeacherOpen}
        onConfirm={(data: CreateClassSubjectTeacherRequest) =>
          assignTeacherMutation.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsAssignTeacherOpen(false)
              },
            },
          )
        }
        isSubmitting={assignTeacherMutation.isPending}
        academicYears={academicYears}
        classes={classes}
      />

      <EditTeacherAssignmentDialog
        assignment={assignmentToEdit} // Removed as any
        open={!!assignmentToEdit}
        onOpenChange={() => setAssignmentToEdit(null)}
        onConfirm={(data: UpdateClassSubjectTeacherRequest) =>
          assignmentToEdit &&
          updateAssignmentMutation.mutate(
            {
              path: {
                class_id: assignmentToEdit.class_id,
                subject_id: assignmentToEdit.subject_id,
                academic_year_id: assignmentToEdit.academic_year_id,
              },
              body: data,
            },
            {
              onSuccess: () => {
                setAssignmentToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateAssignmentMutation.isPending}
      />

      <AlertDialog
        open={!!assignmentToDelete}
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
                assignmentToDelete &&
                removeAssignmentMutation.mutate(
                  {
                    path: {
                      class_id: assignmentToDelete.class_id,
                      subject_id: assignmentToDelete.subject_id,
                      teacher_id: assignmentToDelete.teacher_id,
                      academic_year_id: assignmentToDelete.academic_year_id,
                    },
                  },
                  {
                    onSuccess: () => {
                      setAssignmentToDelete(null)
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
    </div>
  )
}
