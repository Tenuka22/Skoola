import { useMutation, useQueryClient } from '@tanstack/react-query'
import type { StaffDepartmentCreateData, StaffDepartmentUpdateData } from '@/lib/api'
import {
  _StaffDepartmentCreate_,
  _StaffDepartmentDelete_,
  _StaffDepartmentUpdate_,
} from '@/lib/api'
import { authClient } from '@/lib/clients'

export const useCreateStaffDepartment = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (body: StaffDepartmentCreateData['body']) => {
      const response = await _StaffDepartmentCreate_({
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffDepartmentGetAll'] })
    },
  })
}

export const useUpdateStaffDepartment = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({
      path,
      body,
    }: {
      path: { id: string }
      body: StaffDepartmentUpdateData['body']
    }) => {
      const response = await _StaffDepartmentUpdate_({
        path,
        body,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffDepartmentGetAll'] })
    },
  })
}

export const useDeleteStaffDepartment = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async (path: { id: string }) => {
      const response = await _StaffDepartmentDelete_({
        path,
        client: authClient,
      })
      return response.data
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['staffDepartmentGetAll'] })
    },
  })
}
