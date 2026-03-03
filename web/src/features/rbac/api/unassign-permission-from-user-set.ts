import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UnassignPermissionFromUserSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getUserSetPermissionsQueryKey,
  unassignPermissionFromUserSetMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUnassignPermissionFromUserSet = (
  options?: Partial<Options<UnassignPermissionFromUserSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...unassignPermissionFromUserSetMutation({
      client: authClient,
      ...options,
    }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission unassigned from set successfully')
      if (variables.path?.user_set_id) {
        queryClient.invalidateQueries({
          queryKey: getUserSetPermissionsQueryKey({
            path: { user_set_id: variables.path.user_set_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to unassign permission')
    },
  })
}
