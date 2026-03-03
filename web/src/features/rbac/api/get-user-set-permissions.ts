import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetUserSetPermissionsData } from '@/lib/api/types.gen'
import { getUserSetPermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getUserSetPermissionsQueryOptions = (
  options: Options<GetUserSetPermissionsData>,
) => {
  return queryOptions({
    ...getUserSetPermissionsOptions({
      client: authClient,
      ...options,
    }),
  })
}
