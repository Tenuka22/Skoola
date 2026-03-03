import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { CreateStaffData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createStaffMutation,
  getAllStaffQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateStaff = (options?: Partial<Options<CreateStaffData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...createStaffMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Staff member created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStaffQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create staff member')
    },
  })
}
