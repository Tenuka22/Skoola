import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { BulkMarkStudentAttendanceData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { bulkMarkStudentAttendanceMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useMarkStudentAttendanceBulk = (
  options?: Partial<Options<BulkMarkStudentAttendanceData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...bulkMarkStudentAttendanceMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Student attendance marked successfully')
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
      toast.error(error.message || 'Failed to mark student attendance')
    },
  })
}
