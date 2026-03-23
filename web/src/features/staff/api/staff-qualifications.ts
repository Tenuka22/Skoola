import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffQualificationCreateData, StaffQualificationUpdateData } from '@/lib/api'
import {
  _StaffQualificationCreate_,
  _StaffQualificationDelete_,
  _StaffQualificationUpdate_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffQualification = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffQualificationCreateData['body']) => {
      const response = await _StaffQualificationCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffQualificationGetAll'] })
    },
  })
}

export const useUpdateStaffQualification = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      path,
      body,
    }: {
      path: { id: string }
      body: StaffQualificationUpdateData['body']
    }) => {
      const response = await _StaffQualificationUpdate_({
        path,
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffQualificationGetAll'] })
    },
  })
}

export const useDeleteStaffQualification = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffQualificationDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffQualificationGetAll'] })
    },
  })
}
