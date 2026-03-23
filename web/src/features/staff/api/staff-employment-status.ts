import { useMutation, useQueryClient } from '@tanstack/react-query'
import type {
  StaffEmploymentStatusCreateData,
  StaffEmploymentStatusUpdateData,
} from '@/lib/api'
import {
  _StaffEmploymentStatusBulkDelete_,
  _StaffEmploymentStatusCreate_,
  _StaffEmploymentStatusDelete_,
  _StaffEmploymentStatusUpdate_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffEmploymentStatus = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffEmploymentStatusCreateData['body']) => {
      const response = await _StaffEmploymentStatusCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffEmploymentStatusGetAll'] })
    },
  })
}

export const useUpdateStaffEmploymentStatus = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      path,
      body,
    }: {
      path: { id: string }
      body: StaffEmploymentStatusUpdateData['body']
    }) => {
      const response = await _StaffEmploymentStatusUpdate_({
        path,
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffEmploymentStatusGetAll'] })
    },
  })
}

export const useDeleteStaffEmploymentStatus = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffEmploymentStatusDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffEmploymentStatusGetAll'] })
    },
  })
}

export const useBulkDeleteStaffEmploymentStatus = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      const response = await _StaffEmploymentStatusBulkDelete_({
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffEmploymentStatusGetAll'] })
    },
  })
}
