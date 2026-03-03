import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { AssignPermissionToUserSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignPermissionToUserSetMutation,
  getUserSetPermissionsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignPermissionToUserSet = (
  options?: Partial<Options<AssignPermissionToUserSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...assignPermissionToUserSetMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission assigned to set successfully')
      if (variables.path?.user_set_id) {
        queryClient.invalidateQueries({
          queryKey: getUserSetPermissionsQueryKey({
            path: { user_set_id: variables.path.user_set_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission')
    },
  })
}
