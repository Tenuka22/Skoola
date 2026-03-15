import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type AssignPermissionToRoleInput = {
  path: {
    role_id: string
  }
  body: {
    permission: string
  }
}

type AssignPermissionToRoleResponse = {
  success?: boolean
}

const getRolePermissionsQueryKey = (roleId: string) => [
  'role-permissions',
  roleId,
]

export const useAssignPermissionToRole = () => {
  const queryClient = useQueryClient()
  return useMutation<
    AssignPermissionToRoleResponse,
    Error,
    AssignPermissionToRoleInput
  >({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        AssignPermissionToRoleResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/role-sets/{role_id}/permissions',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: (_data, variables) => {
      toast.success('Permission assigned to role successfully')
      if (variables.path.role_id) {
        queryClient.invalidateQueries({
          queryKey: getRolePermissionsQueryKey(variables.path.role_id),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission')
    },
  })
}
