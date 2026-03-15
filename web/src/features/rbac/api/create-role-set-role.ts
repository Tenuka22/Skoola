import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type CreateRoleSetRoleInput = {
  body: {
    role_id: string
    role_set_id: string
  }
}

type CreateRoleSetRoleResponse = {
  success?: boolean
}

const getRoleSetRolesQueryKey = (roleSetId: string) => [
  'role-set-roles',
  roleSetId,
]

export const useCreateRoleSetRole = () => {
  const queryClient = useQueryClient()
  return useMutation<CreateRoleSetRoleResponse, Error, CreateRoleSetRoleInput>({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        CreateRoleSetRoleResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/db/role-set-roles',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: (_data, variables) => {
      toast.success('Role added to role set successfully')
      queryClient.invalidateQueries({
        queryKey: getRoleSetRolesQueryKey(variables.body.role_set_id),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to add role to role set')
    },
  })
}
