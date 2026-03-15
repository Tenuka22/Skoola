import { queryOptions } from '@tanstack/react-query'
import { authClient } from '@/lib/clients'

type PermissionSet = {
  id: string
  name: string
  description?: string
}

type GetPermissionSetsResponse = Array<PermissionSet>

type GetPermissionSetsOptions = {
  query?: Record<string, string | number | boolean | undefined>
}

export const getPermissionSetsQueryOptions = (
  options?: GetPermissionSetsOptions,
) => {
  return queryOptions({
    queryKey: ['permission-sets', options?.query ?? {}],
    queryFn: async () => {
      const result = await authClient.request<
        GetPermissionSetsResponse,
        unknown,
        false,
        'data'
      >({
        url: '/admin/user-sets',
        method: 'GET',
        responseStyle: 'data',
        throwOnError: false,
        query: options?.query,
      })

      return result ?? []
    },
  })
}
