import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { AssignSubjectToGradeData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignSubjectToGradeMutation,
  getAllSubjectsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignSubjectToGrade = (
  options?: Partial<Options<AssignSubjectToGradeData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...assignSubjectToGradeMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Subject assigned to grade successfully')
      queryClient.invalidateQueries({
        queryKey: getAllSubjectsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign subject to grade')
    },
  })
}
