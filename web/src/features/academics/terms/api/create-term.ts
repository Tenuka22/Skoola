import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateTermData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createTermMutation,
  getAllAcademicYearsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateTerm = (options?: Partial<Options<CreateTermData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createTermMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Term created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllAcademicYearsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create term')
    },
  })
}
