import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { authClient } from '@/lib/clients'

type AssignPermissionSetToStaffInput = {
  path: {
    staff_id: string
    set_id: string
  }
}

type AssignPermissionSetToStaffResponse = {
  success?: boolean
}

const getStaffPermissionSetsQueryKey = (staffId: string) => [
  'staff-permission-sets',
  staffId,
]

export const useAssignPermissionSetToStaff = () => {
  const queryClient = useQueryClient()
  return useMutation<
    AssignPermissionSetToStaffResponse,
    Error,
    AssignPermissionSetToStaffInput
  >({
    mutationFn: async (variables) => {
      const result = await authClient.request<
        AssignPermissionSetToStaffResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/staff/{staff_id}/permission-sets/{set_id}',
        method: 'POST',
        responseStyle: 'data',
        throwOnError: false,
        path: variables.path,
      })

      return result ?? {}
    },
    onSuccess: (_data, variables) => {
      toast.success('Permission set assigned successfully')
      if (variables.path.staff_id) {
        queryClient.invalidateQueries({
          queryKey: getStaffPermissionSetsQueryKey(variables.path.staff_id),
        })
      }
    },
    onError: (error) => {
      toast.error(error.message || 'Failed to assign permission set')
    },
  })
}
