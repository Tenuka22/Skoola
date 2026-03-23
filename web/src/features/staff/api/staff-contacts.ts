import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffContactCreateData, StaffContactUpdateData } from '@/lib/api'
import {
  _StaffContactBulkDelete_,
  _StaffContactCreate_,
  _StaffContactDelete_,
  _StaffContactUpdate_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffContact = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffContactCreateData['body']) => {
      const response = await _StaffContactCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContactGetAll'] })
    },
  })
}

export const useUpdateStaffContact = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      path,
      body,
    }: {
      path: { id: string }
      body: StaffContactUpdateData['body']
    }) => {
      const response = await _StaffContactUpdate_({
        path,
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContactGetAll'] })
    },
  })
}

export const useDeleteStaffContact = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffContactDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContactGetAll'] })
    },
  })
}

export const useBulkDeleteStaffContacts = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      const response = await _StaffContactBulkDelete_({
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContactGetAll'] })
    },
  })
}
