import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UpdateProfileData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getProfileQueryKey,
  updateProfileMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateProfile = (
  options?: Partial<Options<UpdateProfileData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateProfileMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Profile updated successfully')
      queryClient.invalidateQueries({
        queryKey: getProfileQueryKey(),
      })
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to update profile')
    },
  })
}
