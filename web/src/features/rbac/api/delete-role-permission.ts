import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { Options, RolePermissionDeleteData } from '@/lib/api'
import { rolePermissionDeleteMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteRolePermission = (
  options?: Partial<Options<RolePermissionDeleteData>>,
) => {
  return useMutation({
    ...rolePermissionDeleteMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Permission removed from role successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to remove permission from role')
    },
  })
}
