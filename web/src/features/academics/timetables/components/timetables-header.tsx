import { HugeiconsIcon } from '@hugeicons/react'
import { CalendarCheckIn01Icon } from '@hugeicons/core-free-icons'
import { useQueries } from '@tanstack/react-query'
import * as React from 'react'
import { useTimetablesStore } from '../store'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  getAllAcademicYearsOptions,
  getAllClassesOptions,
  getTimetableByClassAndDayOptions,
  getTimetableByTeacherOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export function TimetablesHeader() {
  const {
    selectedAcademicYearId,
    selectedClassId,
    selectedDayOfWeek,
    selectedTeacherId,
    viewMode,
  } = useTimetablesStore()

  const [academicYearsQuery, classesQuery] = useQueries({
    queries: [
      {
        ...getAllAcademicYearsOptions({ client: authClient }),
        staleTime: Infinity,
      },
      { ...getAllClassesOptions({ client: authClient }), staleTime: Infinity },
    ],
  })

  const academicYears = academicYearsQuery.data?.data || []
  const classes = classesQuery.data?.data || []

  const timetableQuery = useQueries({
    queries: [
      {
        ...getTimetableByClassAndDayOptions({
          client: authClient,
          path: {
            class_id: selectedClassId ?? '',
            day_of_week: selectedDayOfWeek ?? '',
            academic_year_id: selectedAcademicYearId ?? '',
          },
        }),
        enabled:
          viewMode === 'class' &&
          !!selectedClassId &&
          !!selectedDayOfWeek &&
          !!selectedAcademicYearId,
        staleTime: 5 * 60 * 1000, // 5 minutes
      },
      {
        ...getTimetableByTeacherOptions({
          client: authClient,
          path: {
            teacher_id: selectedTeacherId ?? '',
            academic_year_id: selectedAcademicYearId ?? '',
          },
        }),
        enabled:
          viewMode === 'teacher' &&
          !!selectedTeacherId &&
          !!selectedAcademicYearId,
        staleTime: 5 * 60 * 1000, // 5 minutes
      },
    ],
  })

  const totalEntries =
    (viewMode === 'class'
      ? timetableQuery[0]?.data?.length
      : timetableQuery[1]?.data?.length) ?? 0

  const selectedYearName =
    academicYears.find((ay) => ay.id === selectedAcademicYearId)?.name || 'N/A'
  const selectedClassName =
    classes.find((c) => c.id === selectedClassId)?.section_name || 'N/A'

  const subtitle = React.useMemo(() => {
    if (viewMode === 'class' && selectedClassId && selectedDayOfWeek) {
      return `Timetable for ${selectedClassName} on ${selectedDayOfWeek} in ${selectedYearName}`
    } else if (viewMode === 'teacher' && selectedTeacherId) {
      // TODO: Fetch teacher name here
      return `Timetable for teacher ${selectedTeacherId} in ${selectedYearName}`
    }
    return 'Select filters to view timetable entries.'
  }, [
    viewMode,
    selectedClassId,
    selectedDayOfWeek,
    selectedTeacherId,
    selectedYearName,
    selectedClassName,
  ])

  return (
    <div className="flex flex-col gap-4 p-8">
      <div className="flex items-center justify-between">
        <div className="flex flex-col gap-1">
          <h1 className="text-3xl font-bold tracking-tight">Timetables</h1>
          <p className="text-muted-foreground">{subtitle}</p>
        </div>
      </div>
      <div className="grid auto-rows-min gap-4 md:grid-cols-3">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium">
              Total Timetable Entries
            </CardTitle>
            <HugeiconsIcon
              icon={CalendarCheckIn01Icon}
              className="size-4 text-muted-foreground"
            />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{totalEntries}</div>
            <p className="text-muted-foreground text-xs">
              Entries for selected criteria
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
