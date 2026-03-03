import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UnassignPermissionFromRoleData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getRolePermissionsQueryKey,
  unassignPermissionFromRoleMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUnassignPermissionFromRole = (
  options?: Partial<Options<UnassignPermissionFromRoleData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...unassignPermissionFromRoleMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission unassigned from role successfully')
      if (variables.path?.role_id) {
        queryClient.invalidateQueries({
          queryKey: getRolePermissionsQueryKey({
            path: { role_id: variables.path.role_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to unassign permission')
    },
  })
}
