import { useMutation, useQueryClient } from '@tanstack/react-query'
import { _StaffDelete_ } from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useDeleteStaff = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffGetAll'] })
    },
  })
}
