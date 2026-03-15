import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkUpdateUserData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkUpdateUserMutation,
  getAllUserQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkUpdateUsers = (
  options?: Partial<Options<BulkUpdateUserData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkUpdateUserMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      const count = variables.body?.updates.length ?? 0
      toast.success(
        `Successfully updated ${count} user${count !== 1 ? 's' : ''}.`,
      )
      queryClient.invalidateQueries({
        queryKey: getAllUserQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update users')
    },
  })
}
