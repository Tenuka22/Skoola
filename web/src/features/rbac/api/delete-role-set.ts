import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type DeleteRoleSetInput = {
  path: {
    id: string
  }
}

type DeleteRoleSetResponse = {
  success?: boolean
}

const getAllRoleSetsQueryKey = () => ['role-sets']

export const useDeleteRoleSet = () => {
  const queryClient = useQueryClient()
  return useMutation<DeleteRoleSetResponse, Error, DeleteRoleSetInput>({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        DeleteRoleSetResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/role-sets/{id}',
        method: 'DELETE',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
      })

      return result ?? {}
    },
    onSuccess: () => {
      toast.success('Role set deleted successfully')
      queryClient.invalidateQueries({ queryKey: getAllRoleSetsQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete role set')
    },
  })
}
