import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { Options, UserSetCreateData } from '@/lib/api'
import {
  userSetCreateMutation,
  userSetGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateUserSet = (
  options?: Partial<Options<UserSetCreateData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...userSetCreateMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User set created successfully')
      queryClient.invalidateQueries({ queryKey: userSetGetAllQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create user set')
    },
  })
}
