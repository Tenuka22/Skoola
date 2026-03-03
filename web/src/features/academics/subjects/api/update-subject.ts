import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateSubjectData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllSubjectsQueryKey,
  updateSubjectMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateSubject = (
  options?: Partial<Options<UpdateSubjectData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateSubjectMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Subject updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllSubjectsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update subject')
    },
  })
}
