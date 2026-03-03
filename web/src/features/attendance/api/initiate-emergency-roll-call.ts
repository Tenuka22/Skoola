import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { InitiateEmergencyRollCallData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { initiateEmergencyRollCallMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useInitiateEmergencyRollCall = (
  options?: Partial<Options<InitiateEmergencyRollCallData>>,
) => {
  return useMutation({
    ...initiateEmergencyRollCallMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Emergency roll call initiated successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to initiate roll call')
    },
  })
}
