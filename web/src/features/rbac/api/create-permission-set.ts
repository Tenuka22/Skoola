import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type CreatePermissionSetInput = {
  body: {
    name: string
    description?: string
  }
}

type CreatePermissionSetResponse = {
  id?: string
}

const getAllUserSetQueryKey = () => ['permission-sets']

export const useCreatePermissionSet = () => {
  const queryClient = useQueryClient()
  return useMutation<
    CreatePermissionSetResponse,
    Error,
    CreatePermissionSetInput
  >({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        CreatePermissionSetResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/user-sets',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: () => {
      toast.success('Permission set created successfully')
      queryClient.invalidateQueries({
        queryKey: getAllUserSetQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create permission set')
    },
  })
}
