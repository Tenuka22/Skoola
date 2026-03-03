import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UnassignRoleFromRoleSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getRoleSetRolesQueryKey,
  unassignRoleFromRoleSetMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUnassignRoleFromRoleSet = (
  options?: Partial<Options<UnassignRoleFromRoleSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...unassignRoleFromRoleSetMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Role unassigned from set successfully')
      if (variables.path?.role_set_id) {
        queryClient.invalidateQueries({
          queryKey: getRoleSetRolesQueryKey({
            path: { role_set_id: variables.path.role_set_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to unassign role')
    },
  })
}
