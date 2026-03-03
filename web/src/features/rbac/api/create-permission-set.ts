import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { CreatePermissionSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createPermissionSetMutation,
  getAllPermissionSetsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreatePermissionSet = (
  options?: Partial<Options<CreatePermissionSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createPermissionSetMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Permission set created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllPermissionSetsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create permission set')
    },
  })
}
