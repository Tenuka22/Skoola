import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { AssignClassToTeacherData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignClassToTeacherMutation,
  getAllStaffQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignClassToTeacher = (
  options?: Partial<Options<AssignClassToTeacherData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...assignClassToTeacherMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Class assigned successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStaffQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign class')
    },
  })
}
