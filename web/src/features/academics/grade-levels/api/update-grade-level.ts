import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateGradeLevelData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllGradeLevelsQueryKey,
  updateGradeLevelMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateGradeLevel = (
  options?: Partial<Options<UpdateGradeLevelData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateGradeLevelMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Grade level updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllGradeLevelsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update grade level')
    },
  })
}
