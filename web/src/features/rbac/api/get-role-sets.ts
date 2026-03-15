import { queryOptions } from '@tanstack/react-query'
import { authClient } from '@/lib/clients'

type RoleSet = {
  id: string
  name: string
  description?: string
}

type GetRoleSetsOptions = {
  query?: Record<string, string | number | boolean | undefined>
}

export const getRoleSetsQueryOptions = (options?: GetRoleSetsOptions) => {
  return queryOptions({
    queryKey: ['role-sets', options?.query ?? {}],
    queryFn: async () => {
      const result = await authClient.request<
        Array<RoleSet>,
        unknown,
        false,
        'data'
      >({
        url: '/admin/role-sets',
        method: 'GET',
        responseStyle: 'data',
        throwOnError: false,
        query: options?.query,
      })

      return result ?? []
    },
  })
}
