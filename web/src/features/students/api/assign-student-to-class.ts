import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { AssignStudentToClassData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignStudentToClassMutation,
  getAllStudentsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignStudentToClass = (
  options?: Partial<Options<AssignStudentToClassData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...assignStudentToClassMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Student assigned to class successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStudentsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign student to class')
    },
  })
}
