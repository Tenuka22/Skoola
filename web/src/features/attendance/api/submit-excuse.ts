import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { SubmitExcuseData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { submitExcuseMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useSubmitExcuse = (
  options?: Partial<Options<SubmitExcuseData>>,
) => {
  return useMutation({
    ...submitExcuseMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Excuse submitted successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to submit excuse')
    },
  })
}
