import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UserProfileDeleteData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  userProfileDeleteMutation,
  userProfileGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteUserProfile = (
  options?: Partial<Options<UserProfileDeleteData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...userProfileDeleteMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User profile deleted successfully')
      queryClient.invalidateQueries({
        queryKey: userProfileGetAllQueryKey(),
      })
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to delete user profile')
    },
  })
}
