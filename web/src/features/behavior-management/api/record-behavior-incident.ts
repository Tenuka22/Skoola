import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { RecordBehaviorIncidentData } from '@/lib/api/types.gen'
import type { Options } from '@/lib/api/sdk.gen'
import {
  getStudentBehaviorIncidentsQueryKey,
  recordBehaviorIncidentMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const useRecordBehaviorIncident = (
  options?: Partial<Options<RecordBehaviorIncidentData>>,
) => {
  const queryClient = useQueryClient()
  return useMutation({
    ...recordBehaviorIncidentMutation({ client: authClient, ...options }),
    onSuccess: (_, variables) => {
      toast.success('Behavior incident recorded successfully')
      if (variables.body?.student_id) {
        queryClient.invalidateQueries({
          queryKey: getStudentBehaviorIncidentsQueryKey({
            path: { student_id: variables.body.student_id },
          }),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to record behavior incident')
    },
  })
}
