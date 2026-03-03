import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { BulkAssignStudentsToClassesData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  bulkAssignStudentsToClassesMutation,
  getAllClassesQueryKey,
  getAllStudentsQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useBulkAssignStudentsToClasses = (
  options?: Partial<Options<BulkAssignStudentsToClassesData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...bulkAssignStudentsToClassesMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Students assigned to class successfully')
      queryClient.invalidateQueries({ queryKey: getAllStudentsQueryKey() })
      queryClient.invalidateQueries({ queryKey: getAllClassesQueryKey() })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign students')
    },
  })
}
