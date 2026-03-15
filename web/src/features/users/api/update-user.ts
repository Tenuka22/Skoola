import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateUserData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllUserQueryKey,
  updateUserMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateUser = (options?: Partial<Options<UpdateUserData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateUserMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllUserQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update user')
    },
  })
}
