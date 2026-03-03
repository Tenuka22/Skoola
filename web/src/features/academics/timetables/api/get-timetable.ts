import { queryOptions } from '@tanstack/react-query'
import type { TimetableResponse } from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import {
  getTimetableByClassAndDay,
  getTimetableByTeacher,
} from '@/lib/api/sdk.gen'

export type TimetableViewMode = 'class' | 'teacher'

export interface GetTimetableParams {
  viewMode: TimetableViewMode
  classId?: string
  dayOfWeek?: string
  teacherId?: string
  academicYearId?: string
}

export const getTimetableQueryOptions = ({
  viewMode,
  classId,
  dayOfWeek,
  teacherId,
  academicYearId,
}: GetTimetableParams) => {
  if (viewMode === 'class') {
    const path = {
      class_id: classId ?? '',
      day_of_week: dayOfWeek ?? '',
      academic_year_id: academicYearId ?? '',
    }
    return queryOptions<Array<TimetableResponse>>({
      queryKey: [
        'timetables',
        'class',
        path.class_id,
        path.day_of_week,
        path.academic_year_id,
      ],
      queryFn: async ({ signal }) => {
        if (!path.class_id || !path.day_of_week || !path.academic_year_id) {
          return []
        }
        const res = await getTimetableByClassAndDay({
          client: authClient,
          path,
          signal,
          throwOnError: true,
        })
        return res.data || []
      },
    })
  } else {
    const path = {
      teacher_id: teacherId ?? '',
      academic_year_id: academicYearId ?? '',
    }
    return queryOptions<Array<TimetableResponse>>({
      queryKey: [
        'timetables',
        'teacher',
        path.teacher_id,
        path.academic_year_id,
      ],
      queryFn: async ({ signal }) => {
        if (!path.teacher_id || !path.academic_year_id) {
          return []
        }
        const res = await getTimetableByTeacher({
          client: authClient,
          path,
          signal,
          throwOnError: true,
        })
        return res.data || []
      },
    })
  }
}
