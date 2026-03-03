import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateSubjectTeacherAssignmentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getSubjectsByClassQueryKey,
  updateSubjectTeacherAssignmentMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateSubjectTeacherAssignment = (
  options?: Partial<Options<UpdateSubjectTeacherAssignmentData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateSubjectTeacherAssignmentMutation({
      client: authClient,
      ...options,
    }),
    onSuccess: (_, variables) => {
      toast.success('Teacher assignment updated successfully')
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
      toast.error(error.message || 'Failed to update assignment')
    },
  })
}
