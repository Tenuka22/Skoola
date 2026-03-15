import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type CreateUserSetUserInput = {
  body: {
    user_id: string
    user_set_id: string
  }
}

type CreateUserSetUserResponse = {
  success?: boolean
}

const getUserSetUsersQueryKey = (userSetId: string) => [
  'user-set-users',
  userSetId,
]

export const useCreateUserSetUser = () => {
  const queryClient = useQueryClient()
  return useMutation<CreateUserSetUserResponse, Error, CreateUserSetUserInput>({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        CreateUserSetUserResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/db/user-set-users',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        body: variables.body,
      })

      return result ?? {}
    },
    onSuccess: (_data, variables) => {
      toast.success('User added to permission set successfully')
      queryClient.invalidateQueries({
        queryKey: getUserSetUsersQueryKey(variables.body.user_set_id),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to add user to permission set')
    },
  })
}
