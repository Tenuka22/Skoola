import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffDocumentCreateData } from '@/lib/api'
import { _StaffDocumentCreate_, _StaffDocumentDelete_ } from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffDocument = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffDocumentCreateData['body']) => {
      const response = await _StaffDocumentCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffDocumentGetAll'] })
    },
  })
}

export const useDeleteStaffDocument = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffDocumentDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffDocumentGetAll'] })
    },
  })
}
