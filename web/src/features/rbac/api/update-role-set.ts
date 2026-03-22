import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { RoleSetUpdateData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  roleSetGetAllQueryKey,
  roleSetUpdateMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateRoleSet = (
  options?: Partial<Options<RoleSetUpdateData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...roleSetUpdateMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Role set updated successfully')
      queryClient.invalidateQueries({
        queryKey: roleSetGetAllQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update role set')
    },
  })
}
