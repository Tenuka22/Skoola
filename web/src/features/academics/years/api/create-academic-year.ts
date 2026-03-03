import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateAcademicYearData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createAcademicYearMutation,
  getAllAcademicYearsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateAcademicYear = (
  options?: Partial<Options<CreateAcademicYearData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createAcademicYearMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Academic year created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllAcademicYearsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create academic year')
    },
  })
}
