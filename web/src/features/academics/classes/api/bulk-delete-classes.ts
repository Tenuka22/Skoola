import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkDeleteClassesData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkDeleteClassesMutation,
  getAllClassesQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteClasses = (
  options?: Partial<Options<BulkDeleteClassesData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkDeleteClassesMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      const count = variables.body?.class_ids?.length || 0
      toast.success(`Successfully deleted ${count} classes.`)
      queryClient.invalidateQueries({
        queryKey: getAllClassesQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete classes')
    },
  })
}
