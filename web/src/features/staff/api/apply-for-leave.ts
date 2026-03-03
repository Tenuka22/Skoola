import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { ApplyForLeaveData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { applyForLeaveMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useApplyForLeave = (
  options?: Partial<Options<ApplyForLeaveData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...applyForLeaveMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Leave application submitted successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' && key === 'viewLeaveBalance') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              key._id === 'viewLeaveBalance')
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to apply for leave')
    },
  })
}
