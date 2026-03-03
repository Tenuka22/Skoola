import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkDeleteAcademicYearsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkDeleteAcademicYearsMutation,
  getAllAcademicYearsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteAcademicYears = (
  options?: Partial<Options<BulkDeleteAcademicYearsData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkDeleteAcademicYearsMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      const count = variables.body?.academic_year_ids?.length || 0
      toast.success(`Successfully deleted ${count} academic years.`)
      queryClient.invalidateQueries({
        queryKey: getAllAcademicYearsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete academic years')
    },
  })
}
