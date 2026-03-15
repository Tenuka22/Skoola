import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type DeletePermissionSetInput = {
  path: {
    id: string
  }
}

type DeletePermissionSetResponse = {
  success?: boolean
}

const getAllUserSetQueryKey = () => ['permission-sets']

export const useDeletePermissionSet = () => {
  const queryClient = useQueryClient()
  return useMutation<
    DeletePermissionSetResponse,
    Error,
    DeletePermissionSetInput
  >({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        DeletePermissionSetResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/user-sets/{id}',
        method: 'DELETE',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
      })

      return result ?? {}
    },
    onSuccess: () => {
      toast.success('Permission set deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllUserSetQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete permission set')
    },
  })
}
