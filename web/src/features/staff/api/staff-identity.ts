import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffIdentityCreateData, StaffIdentityUpdateData } from '@/lib/api'
import {
  _StaffIdentityBulkDelete_,
  _StaffIdentityCreate_,
  _StaffIdentityDelete_,
  _StaffIdentityUpdate_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffIdentity = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffIdentityCreateData['body']) => {
      const response = await _StaffIdentityCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffIdentityGetAll'] })
    },
  })
}

export const useUpdateStaffIdentity = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      path,
      body,
    }: {
      path: { id: string }
      body: StaffIdentityUpdateData['body']
    }) => {
      const response = await _StaffIdentityUpdate_({
        path,
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffIdentityGetAll'] })
    },
  })
}

export const useDeleteStaffIdentity = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffIdentityDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffIdentityGetAll'] })
    },
  })
}

export const useBulkDeleteStaffIdentities = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      const response = await _StaffIdentityBulkDelete_({
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffIdentityGetAll'] })
    },
  })
}
