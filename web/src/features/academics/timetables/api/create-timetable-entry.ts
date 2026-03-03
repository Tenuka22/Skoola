import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateTimetableEntryData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createTimetableEntryMutation,
  getTimetableByClassAndDayQueryKey,
  getTimetableByTeacherQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateTimetableEntry = (
  options?: Partial<Options<CreateTimetableEntryData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createTimetableEntryMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      toast.success('Timetable entry created successfully')
      queryClient.invalidateQueries({
        queryKey: getTimetableByClassAndDayQueryKey({
          path: {
            class_id: variables.body?.class_id || '',
            day_of_week: variables.body?.day_of_week || '',
            academic_year_id: variables.body?.academic_year_id || '',
          },
        }),
      })
      queryClient.invalidateQueries({
        queryKey: getTimetableByTeacherQueryKey({
          path: {
            teacher_id: variables.body?.teacher_id || '',
            academic_year_id: variables.body?.academic_year_id || '',
          },
        }),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create timetable entry')
    },
  })
}
