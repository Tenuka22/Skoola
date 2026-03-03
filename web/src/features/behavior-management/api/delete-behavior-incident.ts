import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { DeleteBehaviorIncidentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { deleteBehaviorIncidentMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteBehaviorIncident = (
  options?: Partial<Options<DeleteBehaviorIncidentData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deleteBehaviorIncidentMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Behavior incident deleted successfully')
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
      toast.error(error.message || 'Failed to delete behavior incident')
    },
  })
}
