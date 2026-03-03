import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { BulkUpdateStaffData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkUpdateStaffMutation,
  getAllStaffQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkUpdateStaff = (
  options?: Partial<Options<BulkUpdateStaffData>>,
) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...bulkUpdateStaffMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      const count = variables.body?.staff_ids?.length || 0
      toast.success(`Successfully updated ${count} staff members.`)
      queryClient.invalidateQueries({
        queryKey: getAllStaffQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update staff members')
    },
  })
}
