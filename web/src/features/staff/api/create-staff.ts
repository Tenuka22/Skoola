import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import {
  profileCreateMutation,
  staffGetAllQueryKey,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export interface CreateStaffData {
  name: string
  email?: string
  phone?: string
  dob: string
  gender: string
  employee_id: string
  staff_type: string
  employment_status?: string
  address?: string
  nic?: string
}

export const useCreateStaff = () => {
  const queryClient = useQueryClient()

  return useMutation({
    ...profileCreateMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Staff member created successfully')
      queryClient.invalidateQueries({
        queryKey: staffGetAllQueryKey(),
      })
    },
    onError: (error: Error) => {
      toast.error(error.message || 'Failed to create staff member')
    },
  })
}
