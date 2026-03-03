import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { CreateSubstitutionData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { createSubstitutionMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useCreateSubstitution = (
  options?: Partial<Options<CreateSubstitutionData>>,
) => {
  return useMutation({
    ...createSubstitutionMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Substitution created successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to create substitution')
    },
  })
}
