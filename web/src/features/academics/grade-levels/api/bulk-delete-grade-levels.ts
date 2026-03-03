import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkDeleteGradeLevelsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkDeleteGradeLevelsMutation,
  getAllGradeLevelsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteGradeLevels = (
  options?: Partial<Options<BulkDeleteGradeLevelsData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkDeleteGradeLevelsMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      const count = variables.body?.grade_level_ids?.length || 0
      toast.success(`Successfully deleted ${count} grade levels.`)
      queryClient.invalidateQueries({
        queryKey: getAllGradeLevelsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete grade levels')
    },
  })
}
