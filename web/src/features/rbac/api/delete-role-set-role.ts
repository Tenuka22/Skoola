import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type DeleteRoleSetRoleInput = {
  path: {
    id: string
  }
}

type DeleteRoleSetRoleResponse = {
  success?: boolean
}

export const useDeleteRoleSetRole = () => {
  const queryClient = useQueryClient()
  return useMutation<DeleteRoleSetRoleResponse, Error, DeleteRoleSetRoleInput>({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        DeleteRoleSetRoleResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/db/role-set-roles/{id}',
        method: 'DELETE',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
      })

      return result ?? {}
    },
    onSuccess: (_data) => {
      toast.success('Role removed from role set successfully')
      // Invalidate all role-set-roles queries to refresh lists
      queryClient.invalidateQueries({
        queryKey: ['role-set-roles'],
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to remove role from role set')
    },
  })
}
