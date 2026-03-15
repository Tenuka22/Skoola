import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetAllRoleSetRoleData } from '@/lib/api/types.gen'
import { getAllRoleSetRoleOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllRoleSetRoleQueryOptions = (
  options?: Options<GetAllRoleSetRoleData>,
) => {
  return queryOptions({
    ...getAllRoleSetRoleOptions({
      client: authClient,
      ...options,
    }),
  })
}
