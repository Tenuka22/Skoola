import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { UpdateRoleSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getAllRoleSetsQueryKey,
  updateRoleSetMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateRoleSet = (
  options?: Partial<Options<UpdateRoleSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...updateRoleSetMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Role set updated successfully')
      queryClient.invalidateQueries({ queryKey: getAllRoleSetsQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update role set')
    },
  })
}
