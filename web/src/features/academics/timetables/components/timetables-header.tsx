import { useQueries, useQuery } from '@tanstack/react-query'
import * as React from 'react'
import { useTimetablesSearchParams } from '../search-params'
import {
  getTimetableByClassAndDayQueryOptions,
  getTimetableByTeacherQueryOptions,
} from '../api'
import { getAllAcademicYearsQueryOptions } from '@/features/academics/years/api'
import { getAllClassesQueryOptions } from '@/features/academics/classes/api'
import { HStack, Heading, Stack, Text } from '@/components/primitives'
import { Badge } from '@/components/ui/badge'

interface TimetablesHeaderProps {
  total?: number
}

export function TimetablesHeader({ total: _total }: TimetablesHeaderProps) {
  const {
    selectedAcademicYearId,
    selectedClassId,
    selectedDayOfWeek,
    selectedTeacherId,
    viewMode,
  } = useTimetablesSearchParams()

  const { data: academicYearsData } = useQuery({
    ...getAllAcademicYearsQueryOptions(),
    staleTime: Infinity,
  })

  const { data: classesData } = useQuery({
    ...getAllClassesQueryOptions(),
    staleTime: Infinity,
  })

  const academicYears = academicYearsData?.data || []
  const classes = classesData?.data || []

  const timetableQuery = useQueries({
    queries: [
      {
        ...getTimetableByClassAndDayQueryOptions({
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
        ...getTimetableByTeacherQueryOptions({
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
    if (viewMode === 'class' && selectedClassId) {
      return `Timetable for ${selectedClassName} in ${selectedYearName}`
    } else if (viewMode === 'teacher' && selectedTeacherId) {
      return `Teacher's weekly timetable in ${selectedYearName}`
    }
    return 'Select filters to view timetable entries.'
  }, [
    viewMode,
    selectedClassId,
    selectedTeacherId,
    selectedYearName,
    selectedClassName,
  ])

  return (
    <Stack gap={1}>
      <HStack className="justify-between items-start">
        <HStack>
          <Heading size="h2">Timetables</Heading>
          <Badge
            variant="secondary"
            className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
          >
            {totalEntries} Total Entries
          </Badge>
        </HStack>
      </HStack>
      <Text muted as="p">
        {subtitle}
      </Text>
    </Stack>
  )
}
