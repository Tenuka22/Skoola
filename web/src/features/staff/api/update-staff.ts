import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { UpdateStaffData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllStaffQueryKey,
  getStaffByIdQueryKey,
  updateStaffMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateStaff = (options?: Partial<Options<UpdateStaffData>>) => {
  const queryClient = useQueryClient()

  return useMutation({
    ...updateStaffMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      toast.success('Staff member updated successfully')
      queryClient.invalidateQueries({
        queryKey: getAllStaffQueryKey(),
      })
      if (variables.path?.staff_id) {
        queryClient.invalidateQueries({
          queryKey: getStaffByIdQueryKey({
            path: { staff_id: variables.path.staff_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update staff member')
    },
  })
}
