import { createFileRoute } from '@tanstack/react-router'
import { keepPreviousData, useQueries, useQuery } from '@tanstack/react-query'
import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar02Icon,
  LayoutGridIcon,
  TableIcon,
  User02Icon,
} from '@hugeicons/core-free-icons'

import type { TimetableResponse } from '@/lib/api/types.gen'
import type { TimetableEntryFormValues } from '@/features/academics/timetables/schemas'
import { authClient } from '@/lib/clients'
import {
  getAllAcademicYearsOptions,
  getAllClassesOptions,
  getAllStaffOptions,
  getAllSubjectsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
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
import { useTimetablesSearchParams } from '@/features/academics/timetables/search-params'
import {
  getTimetableQueryOptions,
  useCreateTimetableEntry,
  useDeleteTimetableEntry,
  useUpdateTimetableEntry,
} from '@/features/academics/timetables/api'
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group'
import { cn } from '@/lib/utils'
import { HStack } from '@/components/primitives'

export const Route = createFileRoute('/admin/academics/timetables')({
  component: TimetablesPage,
})

function TimetablesPage() {
  const {
    selectedAcademicYearId,
    setSelectedAcademicYearId,
    selectedClassId,
    setSelectedClassId,
    selectedTeacherId,
    setSelectedTeacherId,
    selectedDayOfWeek,
    setSelectedDayOfWeek,
    viewMode,
    setViewMode,
  } = useTimetablesSearchParams()

  const [isCreateTimetableEntryOpen, setIsCreateTimetableEntryOpen] =
    React.useState(false)
  const [timetableEntryToEdit, setTimetableEntryToEdit] =
    React.useState<TimetableResponse | null>(null)
  const [timetableEntryToDelete, setTimetableEntryToDelete] = React.useState<
    string | null
  >(null)
  const [isGridView, setIsGridView] = React.useState(true)

  // Fetch all academic years, classes, and staff for filters and display
  const [academicYearsQuery, classesQuery, staffQuery, subjectsQuery] =
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
        { ...getAllStaffOptions({ client: authClient }), staleTime: Infinity },
        {
          ...getAllSubjectsOptions({ client: authClient }),
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
  const staff = React.useMemo(
    () => staffQuery.data?.data || [],
    [staffQuery.data],
  )
  const subjects = React.useMemo(
    () => subjectsQuery.data?.data || [],
    [subjectsQuery.data],
  )

  // Set default academic year if not already set
  React.useEffect(() => {
    if (academicYears.length > 0 && !selectedAcademicYearId) {
      const currentYear = academicYears.find((ay) => ay.current)
      setSelectedAcademicYearId(currentYear?.id || academicYears[0]?.id)
    }
  }, [academicYears, selectedAcademicYearId, setSelectedAcademicYearId])

  // Fetch timetable entries based on view mode and selected filters
  const timetableQuery = useQuery({
    ...getTimetableQueryOptions({
      viewMode:
        viewMode === 'class' || viewMode === 'teacher' ? viewMode : 'class',
      classId: selectedClassId ?? undefined,
      dayOfWeek: selectedDayOfWeek ?? undefined,
      teacherId: selectedTeacherId ?? undefined,
      academicYearId: selectedAcademicYearId ?? undefined,
    }),
    enabled:
      !!selectedAcademicYearId &&
      ((viewMode === 'class' && !!selectedClassId && !!selectedDayOfWeek) ||
        (viewMode === 'teacher' && !!selectedTeacherId)),
    placeholderData: keepPreviousData,
  })

  const createMutation = useCreateTimetableEntry()
  const updateMutation = useUpdateTimetableEntry()
  const deleteMutation = useDeleteTimetableEntry()

  const mappedTimetableEntries = React.useMemo(() => {
    if (!timetableQuery.data) return []
    return mapTimetableEntriesForTable(
      timetableQuery.data,
      academicYears,
      classes,
      staff,
      subjects,
    )
  }, [timetableQuery.data, academicYears, classes, staff, subjects])

  const fetchFullData = React.useCallback(async () => {
    return mappedTimetableEntries
  }, [mappedTimetableEntries])

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
        selectedAcademicYearId={selectedAcademicYearId ?? undefined}
        setSelectedAcademicYearId={(val) =>
          setSelectedAcademicYearId(val ?? null)
        }
        selectedClassId={selectedClassId ?? undefined}
        setSelectedClassId={(val) => setSelectedClassId(val ?? null)}
        selectedTeacherId={selectedTeacherId ?? undefined}
        setSelectedTeacherId={(val) => setSelectedTeacherId(val ?? null)}
        selectedDayOfWeek={selectedDayOfWeek ?? undefined}
        setSelectedDayOfWeek={(val) => setSelectedDayOfWeek(val ?? null)}
        viewMode={
          viewMode === 'class' || viewMode === 'teacher' ? viewMode : 'class'
        }
      />
      <TimetablesListContainer
        query={timetableQuery}
        columns={columns}
        data={mappedTimetableEntries}
        isGridView={isGridView}
        viewMode={viewMode || 'class'}
        onEdit={setTimetableEntryToEdit}
        onFetchFullData={fetchFullData}
        onAdd={() => setIsCreateTimetableEntryOpen(true)}
        onAddLabel="Add Entry"
        extraActions={
          <HStack gap={2}>
            <ToggleGroup
              value={[isGridView ? 'grid' : 'list']}
              onValueChange={(val) => {
                const value = Array.isArray(val) ? val[0] : val
                if (value) setIsGridView(value === 'grid')
              }}
              className="border p-1 rounded-lg bg-muted/50"
            >
              <ToggleGroupItem
                value="grid"
                size="sm"
                className="px-2.5 h-7 data-[state=on]:bg-background data-[state=on]:shadow-sm"
                title="Grid View"
              >
                <HugeiconsIcon icon={LayoutGridIcon} className="size-3.5" />
              </ToggleGroupItem>
              <ToggleGroupItem
                value="list"
                size="sm"
                className="px-2.5 h-7 data-[state=on]:bg-background data-[state=on]:shadow-sm"
                title="List View"
              >
                <HugeiconsIcon icon={TableIcon} className="size-3.5" />
              </ToggleGroupItem>
            </ToggleGroup>

            <ToggleGroup
              value={[viewMode || 'class']}
              onValueChange={(val) => {
                const value = Array.isArray(val) ? val[0] : val
                if (value === 'class' || value === 'teacher') setViewMode(value)
              }}
              className="border p-1 rounded-lg bg-muted/50"
            >
              <ToggleGroupItem
                value="class"
                className={cn(
                  'px-3 py-1 h-7 text-[10px] data-[state=on]:bg-primary data-[state=on]:text-primary-foreground',
                  viewMode === 'class' && 'shadow-sm',
                )}
              >
                <HugeiconsIcon icon={Calendar02Icon} className="size-3 mr-1" />
                Class
              </ToggleGroupItem>
              <ToggleGroupItem
                value="teacher"
                className={cn(
                  'px-3 py-1 h-7 text-[10px] data-[state=on]:bg-primary data-[state=on]:text-primary-foreground',
                  viewMode === 'teacher' && 'shadow-sm',
                )}
              >
                <HugeiconsIcon icon={User02Icon} className="size-3 mr-1" />
                Teacher
              </ToggleGroupItem>
            </ToggleGroup>
          </HStack>
        }
      />

      <TimetableAddDialog
        open={isCreateTimetableEntryOpen}
        onOpenChange={setIsCreateTimetableEntryOpen}
        onConfirm={(data: TimetableEntryFormValues) =>
          createMutation.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsCreateTimetableEntryOpen(false)
              },
            },
          )
        }
        isSubmitting={createMutation.isPending}
        academicYears={academicYears}
        classes={classes}
        staff={staff}
      />

      <TimetableEditDialog
        timetableEntry={timetableEntryToEdit}
        open={!!timetableEntryToEdit}
        onOpenChange={() => setTimetableEntryToEdit(null)}
        onConfirm={(data: TimetableEntryFormValues) =>
          timetableEntryToEdit &&
          updateMutation.mutate(
            {
              path: { id: timetableEntryToEdit.id },
              body: data,
            },
            {
              onSuccess: () => {
                setTimetableEntryToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateMutation.isPending}
        academicYears={academicYears}
        classes={classes}
        staff={staff}
      />

      <AlertDialog
        open={!!timetableEntryToDelete}
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
                timetableEntryToDelete &&
                deleteMutation.mutate(
                  {
                    path: { id: timetableEntryToDelete },
                  },
                  {
                    onSuccess: () => {
                      setTimetableEntryToDelete(null)
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
