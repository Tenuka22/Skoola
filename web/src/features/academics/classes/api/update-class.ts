import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateClassData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllClassesQueryKey,
  updateClassMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateClass = (options?: Partial<Options<UpdateClassData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateClassMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Class updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllClassesQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update class')
    },
  })
}
