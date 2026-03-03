import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateAcademicYearData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllAcademicYearsQueryKey,
  updateAcademicYearMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateAcademicYear = (
  options?: Partial<Options<UpdateAcademicYearData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateAcademicYearMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Academic year updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllAcademicYearsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update academic year')
    },
  })
}
