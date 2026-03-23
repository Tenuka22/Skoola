import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffSkillCreateData } from '@/lib/api'
import { _StaffSkillCreate_, _StaffSkillDelete_ } from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffSkill = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffSkillCreateData['body']) => {
      const response = await _StaffSkillCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffSkillGetAll'] })
    },
  })
}

export const useDeleteStaffSkill = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffSkillDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffSkillGetAll'] })
    },
  })
}
