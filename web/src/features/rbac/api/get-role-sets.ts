import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetAllRoleSetsData } from '@/lib/api/types.gen'
import { getAllRoleSetsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getRoleSetsQueryOptions = (
  options?: Options<GetAllRoleSetsData>,
) => {
  return queryOptions({
    ...getAllRoleSetsOptions({
      client: authClient,
      ...options,
    }),
  })
}
