import { queryOptions } from '@tanstack/react-query'
import { authClient } from '@/lib/clients'

type GetRoleSetRolesInput = {
  path: {
    role_set_id: string
  }
}

export const getRoleSetRolesQueryOptions = (
  options: GetRoleSetRolesInput,
) => {
  return queryOptions({
    queryKey: ['role-set-roles', options.path.role_set_id],
    queryFn: async () => {
      const result = await authClient.request<string[], unknown, false, 'data'>(
        {
          url: '/admin/role-sets/{role_set_id}/roles',
          method: 'GET',
          responseStyle: 'data',
          throwOnError: false,
          path: options.path,
        },
      )

      return result ?? []
    },
  })
}
