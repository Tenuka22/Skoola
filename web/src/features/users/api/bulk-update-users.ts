import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkUpdateUsersData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkUpdateUsersMutation,
  getAllUsersQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkUpdateUsers = (
  options?: Partial<Options<BulkUpdateUsersData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkUpdateUsersMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      const count = variables.body?.user_ids?.length || 0
      toast.success(
        `Successfully updated ${count} user${count !== 1 ? 's' : ''}.`,
      )
      queryClient.invalidateQueries({
        queryKey: getAllUsersQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update users')
    },
  })
}
