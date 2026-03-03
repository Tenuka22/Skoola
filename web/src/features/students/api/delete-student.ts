import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteStudentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteStudentMutation,
  getAllStudentsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteStudent = (
  options?: Partial<Options<DeleteStudentData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...deleteStudentMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Student deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStudentsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete student')
    },
  })
}
