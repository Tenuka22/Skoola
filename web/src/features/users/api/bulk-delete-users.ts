import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkDeleteUserData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkDeleteUserMutation,
  getAllUserQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteUsers = (
  options?: Partial<Options<BulkDeleteUserData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkDeleteUserMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      const count = variables.body?.ids.length ?? 0
      toast.success(
        `Successfully deleted ${count} user${count !== 1 ? 's' : ''}.`,
      )
      queryClient.invalidateQueries({
        queryKey: getAllUserQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete users')
    },
  })
}
