import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteSubjectData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteSubjectMutation,
  getAllSubjectsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteSubject = (
  options?: Partial<Options<DeleteSubjectData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...deleteSubjectMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Subject deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllSubjectsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete subject')
    },
  })
}
