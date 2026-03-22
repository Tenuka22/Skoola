import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UserDeleteData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  userDeleteMutation,
  userGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteUser = (options?: Partial<Options<UserDeleteData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...userDeleteMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User deleted successfully')
      queryClient.invalidateQueries({
        queryKey: userGetAllQueryKey(),
      })
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to delete user')
    },
  })
}
