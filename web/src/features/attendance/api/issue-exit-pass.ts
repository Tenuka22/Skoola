import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { IssueExitPassData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { issueExitPassMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useIssueExitPass = (
  options?: Partial<Options<IssueExitPassData>>,
) => {
  return useMutation({
    ...issueExitPassMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Exit pass issued successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to issue exit pass')
    },
  })
}
