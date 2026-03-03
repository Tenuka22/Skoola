import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { AssignSubjectToTeacherData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignSubjectToTeacherMutation,
  getAllStaffQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignSubjectToTeacher = (
  options?: Partial<Options<AssignSubjectToTeacherData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...assignSubjectToTeacherMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Subject assigned successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStaffQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign subject')
    },
  })
}
