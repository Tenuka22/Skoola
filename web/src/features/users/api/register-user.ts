import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { RegisterUserData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllUsersQueryKey,
  getUserStatisticsQueryKey,
  registerUserMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useRegisterUser = (
  options?: Partial<Options<RegisterUserData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...registerUserMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllUsersQueryKey(),
      })
      queryClient.invalidateQueries({
        queryKey: getUserStatisticsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create user')
    },
  })
}
