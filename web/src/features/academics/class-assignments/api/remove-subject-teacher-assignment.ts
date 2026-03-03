import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { RemoveSubjectTeacherAssignmentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getSubjectsByClassQueryKey,
  removeSubjectTeacherAssignmentMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useRemoveSubjectTeacherAssignment = (
  options?: Partial<Options<RemoveSubjectTeacherAssignmentData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...removeSubjectTeacherAssignmentMutation({
      client: authClient,
      ...options,
    }),
    onSuccess: (_, variables) => {
      toast.success('Teacher assignment removed successfully')
      queryClient.invalidateQueries({
        queryKey: getSubjectsByClassQueryKey({
          path: {
            class_id: variables.path?.class_id || '',
            academic_year_id: variables.path?.academic_year_id || '',
          },
        }),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to remove assignment')
    },
  })
}
