import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffEmploymentHistoryCreateData } from '@/lib/api'
import {
  _StaffEmploymentHistoryCreate_,
  _StaffEmploymentHistoryDelete_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffEmploymentHistory = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffEmploymentHistoryCreateData['body']) => {
      const response = await _StaffEmploymentHistoryCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffEmploymentHistoryGetAll'] })
    },
  })
}

export const useDeleteStaffEmploymentHistory = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffEmploymentHistoryDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffEmploymentHistoryGetAll'] })
    },
  })
}
