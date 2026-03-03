import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { DeleteStaffData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deleteStaffMutation,
  getAllStaffQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteStaff = (options?: Partial<Options<DeleteStaffData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...deleteStaffMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Staff member deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStaffQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete staff member')
    },
  })
}
