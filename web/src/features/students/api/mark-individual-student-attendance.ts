import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { MarkIndividualStudentAttendanceData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { markIndividualStudentAttendanceMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useMarkIndividualStudentAttendance = (
  options?: Partial<Options<MarkIndividualStudentAttendanceData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...markIndividualStudentAttendanceMutation({
      client: authClient,
      ...options,
    }),
    onSuccess: () => {
      toast.success('Attendance marked successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' && key === 'getAttendanceByStudent') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              key._id === 'getAttendanceByStudent')
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to mark attendance')
    },
  })
}
