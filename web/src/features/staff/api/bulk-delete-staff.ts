import { useMutation, useQueryClient } from '@tanstack/react-query'
import { _StaffBulkDelete_ } from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useBulkDeleteStaff = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      const response = await _StaffBulkDelete_({
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffGetAll'] })
    },
  })
}
