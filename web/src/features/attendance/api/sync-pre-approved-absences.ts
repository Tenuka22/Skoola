import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { SyncPreApprovedAbsencesData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { syncPreApprovedAbsencesMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useSyncPreApprovedAbsences = (
  options?: Partial<Options<SyncPreApprovedAbsencesData>>,
) => {
  return useMutation({
    ...syncPreApprovedAbsencesMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Pre-approved absences synced successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to sync absences')
    },
  })
}
