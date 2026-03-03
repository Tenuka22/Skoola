import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { SyncSchoolBusinessData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { syncSchoolBusinessMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useSyncSchoolBusiness = (
  options?: Partial<Options<SyncSchoolBusinessData>>,
) => {
  return useMutation({
    ...syncSchoolBusinessMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('School business synced successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to sync school business')
    },
  })
}
