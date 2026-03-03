import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { GetRoleSetRolesData } from '@/lib/api/types.gen'
import { getRoleSetRolesOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getRoleSetRolesQueryOptions = (
  options: Options<GetRoleSetRolesData>,
) => {
  return queryOptions({
    ...getRoleSetRolesOptions({
      client: authClient,
      ...options,
    }),
  })
}
