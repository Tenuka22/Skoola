import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { Options, RoleSetRoleCreateData } from '@/lib/api'
import { roleSetRoleCreateMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateRoleSetRole = (
  options?: Partial<Options<RoleSetRoleCreateData>>,
) => {
  return useMutation({
    ...roleSetRoleCreateMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Role added to role set successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to add role to role set')
    },
  })
}
