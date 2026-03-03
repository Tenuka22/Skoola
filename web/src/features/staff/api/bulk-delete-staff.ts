import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkDeleteStaffData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkDeleteStaffMutation,
  getAllStaffQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkDeleteStaff = (
  options?: Partial<Options<BulkDeleteStaffData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkDeleteStaffMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      const count = variables.body?.staff_ids?.length || 0
      toast.success(`Successfully deleted ${count} staff members.`)
      queryClient.invalidateQueries({
        queryKey: getAllStaffQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete staff members')
    },
  })
}
