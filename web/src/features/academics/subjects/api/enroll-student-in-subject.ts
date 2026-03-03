import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { EnrollStudentInSubjectData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  enrollStudentInSubjectMutation,
  getAllSubjectsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useEnrollStudentInSubject = (
  options?: Partial<Options<EnrollStudentInSubjectData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...enrollStudentInSubjectMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Student enrolled in subject successfully')
      queryClient.invalidateQueries({
        queryKey: getAllSubjectsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to enroll student in subject')
    },
  })
}
