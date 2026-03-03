import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteUserData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteUserMutation,
  getAllUsersQueryKey,
  getUserStatisticsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteUser = (options?: Partial<Options<DeleteUserData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...deleteUserMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllUsersQueryKey(),
      })
      queryClient.invalidateQueries({
        queryKey: getUserStatisticsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete user')
    },
  })
}
