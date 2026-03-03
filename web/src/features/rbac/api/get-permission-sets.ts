import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetAllPermissionSetsData } from '@/lib/api/types.gen'
import { getAllPermissionSetsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getPermissionSetsQueryOptions = (
  options?: Options<GetAllPermissionSetsData>,
) => {
  return queryOptions({
    ...getAllPermissionSetsOptions({
      client: authClient,
      ...options,
    }),
  })
}
