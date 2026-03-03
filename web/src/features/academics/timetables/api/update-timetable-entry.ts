import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateTimetableEntryData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { updateTimetableEntryMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateTimetableEntry = (
  options?: Partial<Options<UpdateTimetableEntryData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateTimetableEntryMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Timetable entry updated successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' && key === 'timetables') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              (key._id === 'getTimetableByClassAndDay' ||
                key._id === 'getTimetableByTeacher'))
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update timetable entry')
    },
  })
}
