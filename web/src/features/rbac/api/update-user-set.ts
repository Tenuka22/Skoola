import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { Options, UserSetUpdateData } from '@/lib/api'
import {
  userSetUpdateMutation,
  userSetGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useUpdateUserSet = (
  options?: Partial<Options<UserSetUpdateData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...userSetUpdateMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User set updated successfully')
      queryClient.invalidateQueries({ queryKey: userSetGetAllQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to update user set')
    },
  })
}
