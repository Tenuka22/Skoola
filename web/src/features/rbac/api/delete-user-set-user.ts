import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type DeleteUserSetUserInput = {
  path: {
    id: string
  }
}

type DeleteUserSetUserResponse = {
  success?: boolean
}

export const useDeleteUserSetUser = () => {
  const queryClient = useQueryClient()
  return useMutation<DeleteUserSetUserResponse, Error, DeleteUserSetUserInput>({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        DeleteUserSetUserResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/db/user-set-users/{id}',
        method: 'DELETE',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
      })

      return result ?? {}
    },
    onSuccess: () => {
      toast.success('User removed from permission set successfully')
      // Invalidate all user-set-users queries to refresh lists
      queryClient.invalidateQueries({
        queryKey: ['user-set-users'],
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to remove user from permission set')
    },
  })
}
