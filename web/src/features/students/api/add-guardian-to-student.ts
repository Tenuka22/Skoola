import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { AddGuardianToStudentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { addGuardianToStudentMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useAddGuardianToStudent = (
  options?: Partial<Options<AddGuardianToStudentData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...addGuardianToStudentMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Guardian added successfully')
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
      toast.error(error.message || 'Failed to add guardian')
    },
  })
}
