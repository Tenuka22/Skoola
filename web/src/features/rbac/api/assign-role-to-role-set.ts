import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type AssignRoleToRoleSetInput = {
  path: {
    role_set_id: string
  }
  body: {
    role_id: string
  }
}

type AssignRoleToRoleSetResponse = {
  success?: boolean
}

const getRoleSetRolesQueryKey = (roleSetId: string) => [
  'role-set-roles',
  roleSetId,
]

export const useAssignRoleToRoleSet = () => {
  const queryClient = useQueryClient()
  return useMutation<
    AssignRoleToRoleSetResponse,
    Error,
    AssignRoleToRoleSetInput
  >({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        AssignRoleToRoleSetResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/role-sets/{role_set_id}/roles',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: (_data, variables) => {
      toast.success('Role assigned to set successfully')
      if (variables.path.role_set_id) {
        queryClient.invalidateQueries({
          queryKey: getRoleSetRolesQueryKey(variables.path.role_set_id),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign role')
    },
  })
}
