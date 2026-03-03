import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UnassignPermissionSetFromStaffData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getStaffPermissionSetsQueryKey,
  unassignPermissionSetFromStaffMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUnassignPermissionSetFromStaff = (
  options?: Partial<Options<UnassignPermissionSetFromStaffData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...unassignPermissionSetFromStaffMutation({
      client: authClient,
      ...options,
    }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission set unassigned successfully')
      if (variables.path?.staff_id) {
        queryClient.invalidateQueries({
          queryKey: getStaffPermissionSetsQueryKey({
            path: { staff_id: variables.path.staff_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to unassign permission set')
    },
  })
}
