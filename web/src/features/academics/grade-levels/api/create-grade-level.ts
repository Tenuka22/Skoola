import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateGradeLevelData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createGradeLevelMutation,
  getAllGradeLevelsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateGradeLevel = (
  options?: Partial<Options<CreateGradeLevelData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createGradeLevelMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Grade level created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllGradeLevelsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create grade level')
    },
  })
}
