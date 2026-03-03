import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkDeleteUsersData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkDeleteUsersMutation,
  getAllUsersQueryKey,
  getUserStatisticsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteUsers = (
  options?: Partial<Options<BulkDeleteUsersData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkDeleteUsersMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      const count = variables.body?.userIds?.length || 0
      toast.success(
        `Successfully deleted ${count} user${count !== 1 ? 's' : ''}.`,
      )
      queryClient.invalidateQueries({
        queryKey: getAllUsersQueryKey(),
      })
      queryClient.invalidateQueries({
        queryKey: getUserStatisticsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete users')
    },
  })
}
