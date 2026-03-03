import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UpdatePermissionSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllPermissionSetsQueryKey,
  updatePermissionSetMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdatePermissionSet = (
  options?: Partial<Options<UpdatePermissionSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updatePermissionSetMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Permission set updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllPermissionSetsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update permission set')
    },
  })
}
