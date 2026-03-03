import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateClassData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createClassMutation,
  getAllClassesQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateClass = (options?: Partial<Options<CreateClassData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createClassMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Class created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllClassesQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create class')
    },
  })
}
