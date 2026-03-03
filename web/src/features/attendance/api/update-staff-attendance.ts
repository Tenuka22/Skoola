import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UpdateStaffAttendanceData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { updateStaffAttendanceMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateStaffAttendance = (
  options?: Partial<Options<UpdateStaffAttendanceData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateStaffAttendanceMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Staff attendance updated successfully')
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
      toast.error(error.message || 'Failed to update staff attendance')
    },
  })
}
