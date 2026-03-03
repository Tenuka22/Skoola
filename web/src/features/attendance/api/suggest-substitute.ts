import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { SuggestSubstituteData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { suggestSubstituteMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useSuggestSubstitute = (
  options?: Partial<Options<SuggestSubstituteData>>,
) => {
  return useMutation({
    ...suggestSubstituteMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Substitute suggestion generated')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to suggest substitute')
    },
  })
}
