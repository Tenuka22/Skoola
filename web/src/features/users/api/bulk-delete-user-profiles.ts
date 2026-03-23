import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UserProfileBulkDeleteData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  userProfileBulkDeleteMutation,
  userProfileGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteUserProfiles = (
  options?: Partial<Options<UserProfileBulkDeleteData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...userProfileBulkDeleteMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User profiles deleted successfully')
      queryClient.invalidateQueries({
        queryKey: userProfileGetAllQueryKey(),
      })
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to delete user profiles')
    },
  })
}
