import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UserProfileCreateData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  userProfileCreateMutation,
  userProfileGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateUserProfile = (
  options?: Partial<Options<UserProfileCreateData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...userProfileCreateMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User profile created successfully')
      queryClient.invalidateQueries({
        queryKey: userProfileGetAllQueryKey(),
      })
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to create user profile')
    },
  })
}
