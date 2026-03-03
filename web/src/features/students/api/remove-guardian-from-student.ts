import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { RemoveGuardianFromStudentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { removeGuardianFromStudentMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useRemoveGuardianFromStudent = (
  options?: Partial<Options<RemoveGuardianFromStudentData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...removeGuardianFromStudentMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Guardian removed successfully')
      queryClient.invalidateQueries({
        predicate: (query) => {
          const key = query.queryKey[0]
          return (
            (typeof key === 'string' && key === 'getAllGuardiansForStudent') ||
            (typeof key === 'object' &&
              key !== null &&
              '_id' in key &&
              key._id === 'getAllGuardiansForStudent')
          )
        },
      })
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to remove guardian')
    },
  })
}
