import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkDeleteSubjectsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkDeleteSubjectsMutation,
  getAllSubjectsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteSubjects = (
  options?: Partial<Options<BulkDeleteSubjectsData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkDeleteSubjectsMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      const count = variables.body?.subject_ids?.length || 0
      toast.success(`Successfully deleted ${count} subjects.`)
      queryClient.invalidateQueries({
        queryKey: getAllSubjectsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete subjects')
    },
  })
}
