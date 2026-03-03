import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteClassData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteClassMutation,
  getAllClassesQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteClass = (options?: Partial<Options<DeleteClassData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...deleteClassMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Class deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllClassesQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete class')
    },
  })
}
