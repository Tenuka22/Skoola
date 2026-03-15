import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/client'
import { authClient } from '@/lib/clients'

type GetUserPermissionsData = {
  path: {
    user_id: string
  }
  url: '/admin/users/{user_id}/permissions'
}

type GetUserPermissionsResponse = {
  permissions: Array<string>
}

const getUserPermissionsQueryKey = (data: GetUserPermissionsData) => [
  'user-permissions',
  data.path.user_id,
]

export const getUserPermissionsQueryOptions = (
  options: Options<GetUserPermissionsData>,
) => {
  const { throwOnError, responseStyle, ...rest } = options

  return queryOptions({
    queryKey: getUserPermissionsQueryKey({
      path: { user_id: options.path?.user_id ?? '' },
      url: '/admin/users/{user_id}/permissions',
    }),
    queryFn: async () => {
      const data = await authClient.request<GetUserPermissionsResponse>({
        url: '/admin/users/{user_id}/permissions',
        method: 'GET',
        responseStyle: 'data',
        throwOnError: false,
        ...rest,
      })

      if (!data) {
        return { permissions: [] }
      }

      return data
    },
  })
}
