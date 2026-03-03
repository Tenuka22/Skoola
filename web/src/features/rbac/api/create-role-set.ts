import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { CreateRoleSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  createRoleSetMutation,
  getAllRoleSetsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateRoleSet = (
  options?: Partial<Options<CreateRoleSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...createRoleSetMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Role set created successfully')
      queryClient.invalidateQueries({ queryKey: getAllRoleSetsQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create role set')
    },
  })
}
