import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { AssignPermissionSetToStaffData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  assignPermissionSetToStaffMutation,
  getStaffPermissionSetsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAssignPermissionSetToStaff = (
  options?: Partial<Options<AssignPermissionSetToStaffData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...assignPermissionSetToStaffMutation({ client: authClient, ...options }),
    onSuccess: (...args) => {
      const [, variables] = args
      toast.success('Permission set assigned successfully')
      if (variables.path?.staff_id) {
        queryClient.invalidateQueries({
          queryKey: getStaffPermissionSetsQueryKey({
            path: { staff_id: variables.path.staff_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission set')
    },
  })
}
