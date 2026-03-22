import { queryOptions } from '@tanstack/react-query'
import type { Options } from '@/lib/api/sdk.gen'
import type { RoleSetRoleGetAllData } from '@/lib/api/types.gen'
import { roleSetRoleGetAllOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export const getAllRoleSetRoleQueryOptions = (
  options?: Options<RoleSetRoleGetAllData>,
) => {
  return queryOptions({
    ...roleSetRoleGetAllOptions({
      client: authClient,
      ...options,
    }),
  })
}
