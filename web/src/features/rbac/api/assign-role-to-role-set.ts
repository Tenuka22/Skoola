import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { AssignRoleToRoleSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignRoleToRoleSetMutation,
  getRoleSetRolesQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignRoleToRoleSet = (
  options?: Partial<Options<AssignRoleToRoleSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...assignRoleToRoleSetMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Role assigned to set successfully')
      if (variables.path?.role_set_id) {
        queryClient.invalidateQueries({
          queryKey: getRoleSetRolesQueryKey({
            path: { role_set_id: variables.path.role_set_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign role')
    },
  })
}
