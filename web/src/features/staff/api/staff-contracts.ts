import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffContractCreateData, StaffContractUpdateData } from '@/lib/api'
import {
  _StaffContractBulkDelete_,
  _StaffContractCreate_,
  _StaffContractDelete_,
  _StaffContractUpdate_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffContract = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffContractCreateData['body']) => {
      const response = await _StaffContractCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContractGetAll'] })
    },
  })
}

export const useUpdateStaffContract = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      path,
      body,
    }: {
      path: { id: string }
      body: StaffContractUpdateData['body']
    }) => {
      const response = await _StaffContractUpdate_({
        path,
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContractGetAll'] })
    },
  })
}

export const useDeleteStaffContract = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffContractDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContractGetAll'] })
    },
  })
}

export const useBulkDeleteStaffContracts = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async () => {
      const response = await _StaffContractBulkDelete_({
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffContractGetAll'] })
    },
  })
}
