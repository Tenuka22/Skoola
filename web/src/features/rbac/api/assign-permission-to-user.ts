import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type AssignPermissionToUserInput = {
  path: {
    user_id: string
  }
  body: {
    permission: string
  }
}

type AssignPermissionToUserResponse = {
  success?: boolean
}

const getUserPermissionsQueryKey = (userId: string) => [
  'user-permissions',
  userId,
]

export const useAssignPermissionToUser = () => {
  const queryClient = useQueryClient()
  return useMutation<
    AssignPermissionToUserResponse,
    Error,
    AssignPermissionToUserInput
  >({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        AssignPermissionToUserResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/users/{user_id}/permissions',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: (_data, variables) => {
      toast.success('Permission assigned to user successfully')
      if (variables.path.user_id) {
        queryClient.invalidateQueries({
          queryKey: getUserPermissionsQueryKey(variables.path.user_id),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission')
    },
  })
}
