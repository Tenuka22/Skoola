import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateStudentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllStudentsQueryKey,
  getStudentByIdQueryKey,
  updateStudentMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateStudent = (
  options?: Partial<Options<UpdateStudentData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateStudentMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      toast.success('Student updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStudentsQueryKey(),
      })
      if (variables.path?.student_id) {
        queryClient.invalidateQueries({
          queryKey: getStudentByIdQueryKey({
            path: { student_id: variables.path.student_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update student')
    },
  })
}
