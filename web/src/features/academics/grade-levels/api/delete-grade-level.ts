import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteGradeLevelData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteGradeLevelMutation,
  getAllGradeLevelsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteGradeLevel = (
  options?: Partial<Options<DeleteGradeLevelData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...deleteGradeLevelMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Grade level deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllGradeLevelsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete grade level')
    },
  })
}
