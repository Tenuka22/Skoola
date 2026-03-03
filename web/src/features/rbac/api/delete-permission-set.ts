import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { DeletePermissionSetData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  deletePermissionSetMutation,
  getAllPermissionSetsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useDeletePermissionSet = (
  options?: Partial<Options<DeletePermissionSetData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...deletePermissionSetMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Permission set deleted successfully')
      queryClient.invalidateQueries({
        queryKey: getAllPermissionSetsQueryKey(),
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to delete permission set')
    },
  })
}
