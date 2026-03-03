import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { AssignSubjectTeacherToClassData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignSubjectTeacherToClassMutation,
  getSubjectsByClassQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignSubjectTeacherToClass = (
  options?: Partial<Options<AssignSubjectTeacherToClassData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...assignSubjectTeacherToClassMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      toast.success('Teacher assigned to class successfully')
      queryClient.invalidateQueries({
        queryKey: getSubjectsByClassQueryKey({
          path: {
            class_id: variables.body?.class_id || '',
            academic_year_id: variables.body?.academic_year_id || '',
          },
        }),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign teacher')
    },
  })
}
