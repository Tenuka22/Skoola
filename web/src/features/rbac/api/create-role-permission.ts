import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { Options, RolePermissionCreateData } from '@/lib/api'
import { rolePermissionCreateMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateRolePermission = (
  options?: Partial<Options<RolePermissionCreateData>>,
) => {
  return useMutation({
    ...rolePermissionCreateMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Permission added to role successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to add permission to role')
    },
  })
}
