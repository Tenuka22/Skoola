import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffMediaCreateData, StaffMediaUpdateData } from '@/lib/api'
import {
  _StaffMediaBulkDelete_,
  _StaffMediaCreate_,
  _StaffMediaDelete_,
  _StaffMediaUpdate_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffMedia = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffMediaCreateData['body']) => {
      const response = await _StaffMediaCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffMediaGetAll'] })
    },
  })
}

export const useUpdateStaffMedia = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      path,
      body,
    }: {
      path: { id: string }
      body: StaffMediaUpdateData['body']
    }) => {
      const response = await _StaffMediaUpdate_({
        path,
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffMediaGetAll'] })
    },
  })
}

export const useDeleteStaffMedia = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffMediaDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffMediaGetAll'] })
    },
  })
}

export const useBulkDeleteStaffMedia = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      const response = await _StaffMediaBulkDelete_({
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffMediaGetAll'] })
    },
  })
}
