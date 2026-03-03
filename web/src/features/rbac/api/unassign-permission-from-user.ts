import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UnassignPermissionFromUserData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getUserPermissionsQueryKey,
  unassignPermissionFromUserMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUnassignPermissionFromUser = (
  options?: Partial<Options<UnassignPermissionFromUserData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...unassignPermissionFromUserMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission unassigned from user successfully')
      if (variables.path?.user_id) {
        queryClient.invalidateQueries({
          queryKey: getUserPermissionsQueryKey({
            path: { user_id: variables.path.user_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to unassign permission')
    },
  })
}
