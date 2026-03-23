import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffCvCreateData } from '@/lib/api'
import { _StaffCvCreate_, _StaffCvDelete_ } from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffCv = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffCvCreateData['body']) => {
      const response = await _StaffCvCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffCvGetAll'] })
    },
  })
}

export const useDeleteStaffCv = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffCvDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffCvGetAll'] })
    },
  })
}
