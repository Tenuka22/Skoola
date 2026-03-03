import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteAcademicYearData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteAcademicYearMutation,
  getAllAcademicYearsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteAcademicYear = (
  options?: Partial<Options<DeleteAcademicYearData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...deleteAcademicYearMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Academic year deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllAcademicYearsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete academic year')
    },
  })
}
