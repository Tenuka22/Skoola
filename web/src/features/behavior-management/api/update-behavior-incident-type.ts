import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateBehaviorIncidentTypeData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllBehaviorIncidentTypesQueryKey,
  updateBehaviorIncidentTypeMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateBehaviorIncidentType = (
  options?: Partial<Options<UpdateBehaviorIncidentTypeData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateBehaviorIncidentTypeMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Behavior incident type updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllBehaviorIncidentTypesQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update behavior incident type')
    },
  })
}
