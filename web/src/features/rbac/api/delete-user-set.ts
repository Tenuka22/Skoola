import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { Options, UserSetDeleteData } from '@/lib/api'
import {
  userSetDeleteMutation,
  userSetGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeleteUserSet = (
  options?: Partial<Options<UserSetDeleteData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...userSetDeleteMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('User set deleted successfully')
      queryClient.invalidateQueries({ queryKey: userSetGetAllQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete user set')
    },
  })
}
