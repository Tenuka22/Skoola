import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type CreateRoleSetInput = {
  body: {
    name: string
    description?: string
  }
}

type CreateRoleSetResponse = {
  id?: string
}

const getAllRoleSetsQueryKey = () => ['role-sets']

export const useCreateRoleSet = () => {
  const queryClient = useQueryClient()
  return useMutation<CreateRoleSetResponse, Error, CreateRoleSetInput>({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        CreateRoleSetResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/role-sets',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: () => {
      toast.success('Role set created successfully')
      queryClient.invalidateQueries({ queryKey: getAllRoleSetsQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create role set')
    },
  })
}
