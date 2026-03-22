import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UserBulkDeleteData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  userBulkDeleteMutation,
  userGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteUsers = (
  options?: Partial<Options<UserBulkDeleteData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...userBulkDeleteMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Successfully deleted users.')
      queryClient.invalidateQueries({
        queryKey: userGetAllQueryKey(),
      })
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to delete users')
    },
  })
}
