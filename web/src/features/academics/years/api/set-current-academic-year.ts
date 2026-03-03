import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { SetCurrentAcademicYearData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllAcademicYearsQueryKey,
  setCurrentAcademicYearMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useSetCurrentAcademicYear = (
  options?: Partial<Options<SetCurrentAcademicYearData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...setCurrentAcademicYearMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Academic year set as current')
      queryClient.invalidateQueries({
        queryKey: getAllAcademicYearsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to set academic year')
    },
  })
}
