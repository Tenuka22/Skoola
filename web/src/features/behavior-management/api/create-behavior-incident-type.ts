import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateBehaviorIncidentTypeData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createBehaviorIncidentTypeMutation,
  getAllBehaviorIncidentTypesQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateBehaviorIncidentType = (
  options?: Partial<Options<CreateBehaviorIncidentTypeData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createBehaviorIncidentTypeMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Behavior incident type created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllBehaviorIncidentTypesQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create behavior incident type')
    },
  })
}
