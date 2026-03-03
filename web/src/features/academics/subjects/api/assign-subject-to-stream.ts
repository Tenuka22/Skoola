import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { AssignSubjectToStreamData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignSubjectToStreamMutation,
  getAllSubjectsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignSubjectToStream = (
  options?: Partial<Options<AssignSubjectToStreamData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...assignSubjectToStreamMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Subject assigned to stream successfully')
      queryClient.invalidateQueries({
        queryKey: getAllSubjectsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign subject to stream')
    },
  })
}
