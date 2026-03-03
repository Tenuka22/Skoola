import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateStudentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createStudentMutation,
  getAllStudentsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateStudent = (
  options?: Partial<Options<CreateStudentData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createStudentMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Student created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStudentsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create student')
    },
  })
}
