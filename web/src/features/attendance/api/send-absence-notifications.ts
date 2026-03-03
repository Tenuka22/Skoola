import { useMutation } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { SendAbsenceNotificationsData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import { sendAbsenceNotificationsMutation } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useSendAbsenceNotifications = (
  options?: Partial<Options<SendAbsenceNotificationsData>>,
) => {
  return useMutation({
    ...sendAbsenceNotificationsMutation({ client: authClient, ...options }),
    onSuccess: () => {
      toast.success('Absence notifications sent successfully')
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to send notifications')
    },
  })
}
