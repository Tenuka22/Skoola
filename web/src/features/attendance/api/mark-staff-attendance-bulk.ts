import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { MarkStaffAttendanceBulkData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { markStaffAttendanceBulkMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useMarkStaffAttendanceBulk = (
  options?: Partial<Options<MarkStaffAttendanceBulkData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...markStaffAttendanceBulkMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Staff attendance marked successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' && key === 'getStaffAttendanceByDate') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              key._id === 'getStaffAttendanceByDate')
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to mark staff attendance')
    },
  })
}
