import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteBehaviorIncidentTypeData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteBehaviorIncidentTypeMutation,
  getAllBehaviorIncidentTypesQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteBehaviorIncidentType = (
  options?: Partial<Options<DeleteBehaviorIncidentTypeData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deleteBehaviorIncidentTypeMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Behavior incident type deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllBehaviorIncidentTypesQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete behavior incident type')
    },
  })
}
