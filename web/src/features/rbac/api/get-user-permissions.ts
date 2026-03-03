import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetUserPermissionsData } from '@/lib/api/types.gen'
import { getUserPermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserPermissionsQueryOptions = (
  options: Options<GetUserPermissionsData>,
) => {
  return queryOptions({
    ...getUserPermissionsOptions({
      client: authClient,
      ...options,
    }),
  })
}
