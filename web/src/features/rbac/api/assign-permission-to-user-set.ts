import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type AssignPermissionToUserSetInput = {
  path: {
    user_set_id: string
  }
  body: {
    permission: string
  }
}

type AssignPermissionToUserSetResponse = {
  success?: boolean
}

const getUserSetPermissionsQueryKey = (userSetId: string) => [
  'user-set-permissions',
  userSetId,
]

export const useAssignPermissionToUserSet = () => {
  const queryClient = useQueryClient()
  return useMutation<
    AssignPermissionToUserSetResponse,
    Error,
    AssignPermissionToUserSetInput
  >({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        AssignPermissionToUserSetResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/user-sets/{user_set_id}/permissions',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: (_data, variables) => {
      toast.success('Permission assigned to set successfully')
      if (variables.path.user_set_id) {
        queryClient.invalidateQueries({
          queryKey: getUserSetPermissionsQueryKey(variables.path.user_set_id),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission')
    },
  })
}
