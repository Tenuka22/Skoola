import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateSubjectData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createSubjectMutation,
  getAllSubjectsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateSubject = (
  options?: Partial<Options<CreateSubjectData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createSubjectMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Subject created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllSubjectsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create subject')
    },
  })
}
