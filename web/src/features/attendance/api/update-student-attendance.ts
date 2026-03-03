import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UpdateStudentAttendanceData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { updateStudentAttendanceMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateStudentAttendance = (
  options?: Partial<Options<UpdateStudentAttendanceData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateStudentAttendanceMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Student attendance updated successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' && key === 'getEnrichedStudentList') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              key._id === 'getEnrichedStudentList')
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update student attendance')
    },
  })
}
