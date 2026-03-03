import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UpdateBehaviorIncidentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { updateBehaviorIncidentMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateBehaviorIncident = (
  options?: Partial<Options<UpdateBehaviorIncidentData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateBehaviorIncidentMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Behavior incident updated successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' &&
              key === 'getStudentBehaviorIncidents') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              key._id === 'getStudentBehaviorIncidents')
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update behavior incident')
    },
  })
}
