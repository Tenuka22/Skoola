import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { AssignPermissionToRoleData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignPermissionToRoleMutation,
  getRolePermissionsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignPermissionToRole = (
  options?: Partial<Options<AssignPermissionToRoleData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...assignPermissionToRoleMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission assigned to role successfully')
      if (variables.path?.role_id) {
        queryClient.invalidateQueries({
          queryKey: getRolePermissionsQueryKey({
            path: { role_id: variables.path.role_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission')
    },
  })
}
