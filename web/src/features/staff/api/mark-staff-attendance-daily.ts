import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { MarkStaffAttendanceDailyData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { markStaffAttendanceDailyMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useMarkStaffAttendanceDaily = (
  options?: Partial<Options<MarkStaffAttendanceDailyData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...markStaffAttendanceDailyMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' &&
              key === 'getStaffAttendanceByStaffMember') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              key._id === 'getStaffAttendanceByStaffMember')
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to mark attendance')
    },
  })
}
