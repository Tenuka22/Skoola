import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { AssignPermissionToUserData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignPermissionToUserMutation,
  getUserPermissionsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignPermissionToUser = (
  options?: Partial<Options<AssignPermissionToUserData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...assignPermissionToUserMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission assigned to user successfully')
      if (variables.path?.user_id) {
        queryClient.invalidateQueries({
          queryKey: getUserPermissionsQueryKey({
            path: { user_id: variables.path.user_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission')
    },
  })
}
