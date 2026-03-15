import { queryOptions } from '@tanstack/react-query'
import { authClient } from '@/lib/clients'

type PermissionSet = {
  id: string
  name: string
  description?: string
}

type GetStaffPermissionSetsInput = {
  path: {
    staff_id: string
  }
}

export const getStaffPermissionSetsQueryOptions = (
  options: GetStaffPermissionSetsInput,
) => {
  return queryOptions({
    queryKey: ['staff-permission-sets', options.path.staff_id],
    queryFn: async () => {
      const result = await authClient.request<
        Array<PermissionSet>,
        unknown,
        false,
        'data'
      >({
        url: '/admin/staff/{staff_id}/permission-sets',
        method: 'GET',
        responseStyle: 'data',
        throwOnError: false,
        path: options.path,
      })

      return result ?? []
    },
  })
}
