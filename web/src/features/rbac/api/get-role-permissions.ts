import { authClient } from '@/lib/clients'
import { queryOptions } from '@tanstack/react-query'

type GetRolePermissionsInput = {
  path: {
    role_id: string
  }
}

type GetRolePermissionsResponse = {
  permissions: string[]
}

export const getRolePermissionsQueryOptions = (
  options: GetRolePermissionsInput,
) => {
  return queryOptions({
    queryKey: ['role-permissions', options.path.role_id],
    queryFn: async () => {
      const result = await authClient.request<
        GetRolePermissionsResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/role-sets/{role_id}/permissions',
        method: 'GET',
        responseStyle: 'data',
        throwOnError: false,
        path: options.path,
      })

      return result ?? { permissions: [] }
    },
  })
}
