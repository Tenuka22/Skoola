import { createFileRoute } from '@tanstack/react-router'
import {
  keepPreviousData,
  useMutation,
  useQueries,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import * as React from 'react'
import { toast } from 'sonner'

import type { TimetableResponse } from '@/lib/api/types.gen'
import type { TimetableEntryFormValues } from '@/features/academics/timetables/schemas'
import { authClient } from '@/lib/clients'
import { handleExportCSV } from '@/lib/export'
import {
  createTimetableEntryMutation as createTimetableEntryFn,
  deleteTimetableEntryMutation as deleteTimetableEntryFn,
  getAllAcademicYearsOptions,
  getAllClassesOptions,
  getAllStaffOptions,
  getTimetableByClassAndDayQueryKey,
  getTimetableByTeacherQueryKey,
  updateTimetableEntryMutation as updateTimetableEntryFn,
} from '@/lib/api/@tanstack/react-query.gen'
import {
  getTimetableByClassAndDay,
  getTimetableByTeacher,
} from '@/lib/api/sdk.gen'
import { useTimetablesStore } from '@/features/academics/timetables/store'
import { TimetablesHeader } from '@/features/academics/timetables/components/timetables-header'
import { TimetablesToolbar } from '@/features/academics/timetables/components/timetables-toolbar'
import { TimetablesListContainer } from '@/features/academics/timetables/components/timetables-list-container'
import {
  getTimetableColumns,
  mapTimetableEntriesForTable,
} from '@/features/academics/timetables/components/timetables-table-columns'
import { TimetableAddDialog } from '@/features/academics/timetables/components/timetable-add-dialog'
import { TimetableEditDialog } from '@/features/academics/timetables/components/timetable-edit-dialog'
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

export const Route = createFileRoute('/admin/academics/timetables')({
  component: TimetablesPage,
})

function TimetablesPage() {
  const store = useTimetablesStore()
  const { search, setDebouncedSearch } = store

  React.useEffect(() => {
    const handler = setTimeout(() => {
      setDebouncedSearch(search)
    }, 400)
    return () => clearTimeout(handler)
  }, [search, setDebouncedSearch])

  const {
    selectedAcademicYearId,
    setSelectedAcademicYearId,
    selectedClassId,
    setSelectedClassId,
    selectedTeacherId,
    setSelectedTeacherId,
    selectedDayOfWeek,
    setSelectedDayOfWeek,
    setIsCreateTimetableEntryOpen,
    setTimetableEntryToEdit,
    setTimetableEntryToDelete,
    viewMode,
    setViewMode,
  } = store

  // Fetch all academic years, classes, and staff for filters and display
  const [academicYearsQuery, classesQuery, staffQuery] = useQueries({
    queries: [
      { ...getAllAcademicYearsOptions({ client: authClient }), staleTime: Infinity },
      { ...getAllClassesOptions({ client: authClient }), staleTime: Infinity },
      { ...getAllStaffOptions({ client: authClient }), staleTime: Infinity },
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
  const staff = React.useMemo(
    () => (staffQuery.data)?.data || [],
    [staffQuery.data],
  )

  // Set default academic year if not already set
  React.useEffect(() => {
    if (academicYears.length > 0 && !selectedAcademicYearId) {
      const currentYear = academicYears.find((ay) => ay.current)
      setSelectedAcademicYearId(currentYear?.id || academicYears[0]?.id)
    }
  }, [academicYears, selectedAcademicYearId, setSelectedAcademicYearId])

  // Fetch timetable entries based on view mode and selected filters
  const timetableQuery = useQuery<Array<TimetableResponse>, Error>({
    queryKey: viewMode === 'class' 
      ? getTimetableByClassAndDayQueryKey({
          client: authClient,
          path: {
            class_id: selectedClassId ?? '',
            day_of_week: selectedDayOfWeek ?? '',
            academic_year_id: selectedAcademicYearId ?? '',
          },
        })
      : getTimetableByTeacherQueryKey({
          client: authClient,
          path: {
            teacher_id: selectedTeacherId ?? '',
            academic_year_id: selectedAcademicYearId ?? '',
          },
        }),
    queryFn: async ({ signal }) => {
      if (viewMode === 'class') {
        if (!selectedClassId || !selectedDayOfWeek || !selectedAcademicYearId) return []
        const res = await getTimetableByClassAndDay({
          client: authClient,
          path: {
            class_id: selectedClassId,
            day_of_week: selectedDayOfWeek,
            academic_year_id: selectedAcademicYearId,
          },
          signal,
          throwOnError: true,
        })
        return res.data || []
      } else {
        if (!selectedTeacherId || !selectedAcademicYearId) return []
        const res = await getTimetableByTeacher({
          client: authClient,
          path: {
            teacher_id: selectedTeacherId,
            academic_year_id: selectedAcademicYearId,
          },
          signal,
          throwOnError: true,
        })
        return res.data || []
      }
    },
    enabled:
      !!selectedAcademicYearId &&
      ((viewMode === 'class' && !!selectedClassId && !!selectedDayOfWeek) ||
        (viewMode === 'teacher' && !!selectedTeacherId)),
    placeholderData: keepPreviousData,
  })

  const queryClient = useQueryClient()
  const invalidateQueries = () => {
    queryClient.invalidateQueries({
      queryKey: getTimetableByClassAndDayQueryKey({
        client: authClient,
        path: {
          class_id: selectedClassId ?? '',
          day_of_week: selectedDayOfWeek ?? '',
          academic_year_id: selectedAcademicYearId ?? '',
        },
      }),
    })
    queryClient.invalidateQueries({
      queryKey: getTimetableByTeacherQueryKey({
        client: authClient,
        path: {
          teacher_id: selectedTeacherId ?? '',
          academic_year_id: selectedAcademicYearId ?? '',
        },
      }),
    })
  }

  const createMutation = useMutation({
    ...createTimetableEntryFn({ client: authClient }),
    onSuccess: () => {
      toast.success('Timetable entry created successfully.')
      invalidateQueries()
      setIsCreateTimetableEntryOpen(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to create timetable entry: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateMutation = useMutation({
    ...updateTimetableEntryFn({ client: authClient }),
    onSuccess: () => {
      toast.success('Timetable entry updated successfully.')
      invalidateQueries()
      setTimetableEntryToEdit(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to update timetable entry: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const deleteMutation = useMutation({
    ...deleteTimetableEntryFn({ client: authClient }),
    onSuccess: () => {
      toast.success('Timetable entry deleted successfully.')
      invalidateQueries()
      setTimetableEntryToDelete(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to delete timetable entry: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const mappedTimetableEntries = React.useMemo(() => {
    if (!timetableQuery.data) return []
    return mapTimetableEntriesForTable(
      timetableQuery.data,
      academicYears,
      classes,
      staff,
    )
  }, [timetableQuery.data, academicYears, classes, staff])

  const columns = getTimetableColumns({
    onEdit: setTimetableEntryToEdit,
    onDelete: (id) => setTimetableEntryToDelete(id),
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <TimetablesHeader />
      <TimetablesToolbar
        academicYears={academicYears}
        classes={classes}
        staff={staff}
        selectedAcademicYearId={selectedAcademicYearId}
        setSelectedAcademicYearId={setSelectedAcademicYearId}
        selectedClassId={selectedClassId}
        setSelectedClassId={setSelectedClassId}
        selectedTeacherId={selectedTeacherId}
        setSelectedTeacherId={setSelectedTeacherId}
        selectedDayOfWeek={selectedDayOfWeek}
        setSelectedDayOfWeek={setSelectedDayOfWeek}
        viewMode={viewMode}
        setViewMode={setViewMode}
        onExport={() =>
          handleExportCSV(
            mappedTimetableEntries,
            'timetables_export.csv',
            [
              { header: 'Class', accessor: 'className' },
              { header: 'Subject', accessor: 'subjectName' },
              { header: 'Teacher', accessor: 'teacherName' },
              { header: 'Day', accessor: 'dayOfWeek' },
              { header: 'Start Time', accessor: 'startTime' },
              { header: 'End Time', accessor: 'endTime' },
              { header: 'Academic Year', accessor: 'academicYearName' },
            ],
          )
        }
      />
      <TimetablesListContainer
        query={timetableQuery}
        columns={columns}
        data={mappedTimetableEntries}
      />

      <TimetableAddDialog
        open={store.isCreateTimetableEntryOpen}
        onOpenChange={setIsCreateTimetableEntryOpen}
        onConfirm={(data: TimetableEntryFormValues) =>
          createMutation.mutate({ body: data })
        }
        isSubmitting={createMutation.isPending}
        academicYears={academicYears}
        classes={classes}
        staff={staff}
      />

      <TimetableEditDialog
        timetableEntry={store.timetableEntryToEdit}
        open={!!store.timetableEntryToEdit}
        onOpenChange={() => setTimetableEntryToEdit(null)}
        onConfirm={(data: TimetableEntryFormValues) =>
          store.timetableEntryToEdit &&
          updateMutation.mutate({
            path: { id: store.timetableEntryToEdit.id },
            body: data,
          })
        }
        isSubmitting={updateMutation.isPending}
        academicYears={academicYears}
        classes={classes}
        staff={staff}
      />

      <AlertDialog
        open={!!store.timetableEntryToDelete}
        onOpenChange={() => setTimetableEntryToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              timetable entry.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                store.timetableEntryToDelete &&
                deleteMutation.mutate({
                  path: { id: store.timetableEntryToDelete },
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
